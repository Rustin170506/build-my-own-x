package shardmaster

import (
	"sync/atomic"
	"time"

	"../raft"
)
import "../labrpc"
import "sync"
import "../labgob"

type OptType int

const (
	Join OptType = iota
	Leave
	Move
	Query
)

type Op struct {
	Type      OptType
	ClientID  int64
	RequestID int64
	// new GID -> servers mappings, for Join
	Servers map[int][]string
	// GIDs, for Leave
	GIDs []int
	// shard, for Move
	Shard int
	// GID, for Move
	GID int
}

const (
	// timeout is the timeout for an op.
	timeout = 100 * time.Millisecond
)

type ShardMaster struct {
	mu      sync.Mutex
	me      int
	rf      *raft.Raft
	applyCh chan raft.ApplyMsg
	dead    int32 // set by Kill()

	configs          []Config        // indexed by config num
	opMap            map[int]chan Op // CommandIndex -> Channel
	lastRequestIDMap map[int64]int64 // ClientID -> RequestID
}

func (sm *ShardMaster) Join(args *JoinArgs, reply *JoinReply) {
	// Your code here.
}

func (sm *ShardMaster) Leave(args *LeaveArgs, reply *LeaveReply) {
	// Your code here.
}

func (sm *ShardMaster) Move(args *MoveArgs, reply *MoveReply) {
	// Your code here.
}

func (sm *ShardMaster) Query(args *QueryArgs, reply *QueryReply) {
	op := Op{
		Type:      Query,
		ClientID:  args.ClientID,
		RequestID: args.RequestID,
	}
	isLeader := sm.startOp(op, timeout)
	if !isLeader {
		reply.WrongLeader = true
		reply.Err = ErrWrongLeader
		return
	}

	reply.Config = sm.executeQuery(args.Num)
	DPrintf("[%d] query config: %v", sm.me, reply.Config)
	reply.Err = OK
}

func (sm *ShardMaster) executeQuery(num int) Config {
	sm.mu.Lock()
	defer sm.mu.Unlock()

	length := len(sm.configs)
	var c Config
	if num == -1 || num >= length {
		c = sm.configs[length-1]
	} else {
		c = sm.configs[num]
	}

	newGroups := deepCopy(c.Groups)
	newConfig := Config{
		Num:    c.Num,
		Shards: c.Shards,
		Groups: newGroups,
	}
	return newConfig
}

func deepCopy(groups map[int][]string) map[int][]string {
	newGroups := make(map[int][]string)
	for gid, servers := range groups {
		newServers := make([]string, len(servers))
		copy(newServers, servers)
		newGroups[gid] = newServers
	}
	return newGroups
}

func (sm *ShardMaster) startOp(op Op, timeout time.Duration) (isLeader bool) {
	sm.mu.Lock()
	index, _, isLeader := sm.rf.Start(op)
	if !isLeader {
		sm.mu.Unlock()
		return
	}
	if _, ok := sm.opMap[index]; !ok {
		DPrintf("[%d] create op channel, index: %d, op: %v", sm.me, index, op)
		sm.opMap[index] = make(chan Op, 1)
	}

	ch := sm.opMap[index]
	sm.mu.Unlock()

	select {
	case opRecv := <-ch:
		{
			sm.mu.Lock()
			delete(sm.opMap, index)
			DPrintf("[%d] drop the op channel, index: %d", sm.me, index)
			if opRecv.RequestID != op.RequestID || opRecv.ClientID != op.ClientID {
				DPrintf("[%d] receive not match opRecv: %v, op: %v", sm.me, opRecv, op)
				isLeader = false
			}
			sm.mu.Unlock()
		}
	case <-time.After(timeout):
		DPrintf("[%d] timeout, index: %d, op: %v", sm.me, index, op)
		isLeader = false
	}

	return isLeader
}

func (sm *ShardMaster) receive() {
	for msg := range sm.applyCh {
		DPrintf("[%d] receive op...", sm.me)
		sm.mu.Lock()
		op := msg.Command.(Op)
		lastRequestID, ok := sm.lastRequestIDMap[op.ClientID]
		if !ok || lastRequestID < op.RequestID {
			DPrintf("[%d] update last request ID, Client: %d, Request: %d", sm.me, op.ClientID, op.RequestID)
			sm.lastRequestIDMap[op.ClientID] = op.RequestID
			switch op.Type {
			case Query:
				// do nothing
			default:
				DPrintf("unknown type op: %s", op.Type)
			}
		}
		ch, ok := sm.opMap[msg.CommandIndex]
		sm.mu.Unlock()
		if ok {
			ch <- op
		} else {
			DPrintf("[%d] can not find op channel, index: %d", sm.me, msg.CommandIndex)
		}
		DPrintf("[%d] receive op done...", sm.me)
	}
}

//
// the tester calls Kill() when a ShardMaster instance won't
// be needed again. you are not required to do anything
// in Kill(), but it might be convenient to (for example)
// turn off debug output from this instance.
//
func (sm *ShardMaster) Kill() {
	atomic.StoreInt32(&sm.dead, 1)
	sm.rf.Kill()
}

// needed by shardkv tester
func (sm *ShardMaster) Raft() *raft.Raft {
	return sm.rf
}

//
// servers[] contains the ports of the set of
// servers that will cooperate via Paxos to
// form the fault-tolerant shardmaster service.
// me is the index of the current server in servers[].
//
func StartServer(servers []*labrpc.ClientEnd, me int, persister *raft.Persister) *ShardMaster {
	labgob.Register(Op{})

	sm := new(ShardMaster)
	sm.me = me

	sm.configs = make([]Config, 1)
	sm.configs[0].Groups = map[int][]string{}
	sm.lastRequestIDMap = make(map[int64]int64)
	sm.opMap = make(map[int]chan Op)

	sm.applyCh = make(chan raft.ApplyMsg)
	sm.rf = raft.Make(servers, me, persister, sm.applyCh)

	go sm.receive()

	return sm
}
