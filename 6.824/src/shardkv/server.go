package shardkv

import (
	"sync/atomic"

	"../shardmaster"
)
import "../labrpc"
import "../raft"
import "sync"
import "../labgob"

type OperationContext struct {
	CommandId int64
	Response  CommandResponse
}

type ShardKV struct {
	mu      sync.Mutex
	me      int
	rf      *raft.Raft
	applyCh chan raft.ApplyMsg
	dead    int32 // set by Kill()

	make_end func(string) *labrpc.ClientEnd
	gid      int
	sc       *shardmaster.Clerk

	maxRaftState int // snapshot if log grows this big
	lastApplied  int // record the lastApplied to prevent stateMachine from rollback

	lastConfig    shardmaster.Config
	currentConfig shardmaster.Config

	stateMachines  map[int]*Shard                // KV stateMachines
	lastOperations map[int64]OperationContext    // determine whether log is duplicated by recording the last commandId and response corresponding to the clientId
	notifyChans    map[int]chan *CommandResponse // notify client goroutine by applier goroutine to response
}

func (kv *ShardKV) Get(args *GetArgs, reply *GetReply) {
	// Your code here.
}

func (kv *ShardKV) PutAppend(args *PutAppendArgs, reply *PutAppendReply) {
	// Your code here.
}

// a dedicated applier goroutine to apply committed entries to stateMachine, take snapshot and apply snapshot from raft
func (kv *ShardKV) applier() {
	for kv.killed() == false {
		select {
		case msg := <-kv.applyCh:
			DPrintf("[Node %d] [Group %v]receive message: %v", kv.me, kv.gid, msg)
			if msg.CommandValid {
				kv.mu.Lock()
				if msg.CommandIndex <= kv.lastApplied {
					DPrintf("[Node %d] [Group %v] discards outdated message because a newer snapshot which lastApplied is %v has been restored", kv.me, kv.gid, kv.lastApplied)
					kv.mu.Unlock()
					continue
				}
				kv.lastApplied = msg.CommandIndex
				var response *CommandResponse
				command := msg.Command.(Command)
				switch command.Op {
				case Operation:
				case Configuration:
				case InsertShards:
				case DeleteShards:
				case EmptyEntry:
					// TODO
				}
				// only notify related channel for currentTerm's log when node is leader
				if _, isLeader := kv.rf.GetState(); isLeader {
					ch := kv.getNotifyChan(msg.CommandIndex)
					ch <- response
				}

				//needSnapshot := kv.needSnapshot()
				//if needSnapshot {
				//	kv.takeSnapshot(msg.CommandIndex)
				//}
				kv.mu.Unlock()
			} else {
				kv.mu.Lock()
				//if kv.rf.CondInstallSnapshot(message.SnapshotTerm, message.SnapshotIndex, message.Snapshot) {
				//	kv.restoreSnapshot(msg.Snapshot)
				//	kv.lastApplied = msg.CommandIndex
				//}
				kv.mu.Unlock()
			}
		}
	}
}

func (kv *ShardKV) getNotifyChan(index int) chan *CommandResponse {
	if _, ok := kv.notifyChans[index]; !ok {
		kv.notifyChans[index] = make(chan *CommandResponse, 1)
	}
	return kv.notifyChans[index]
}

//
// the tester calls Kill() when a ShardKV instance won't
// be needed again. you are not required to do anything
// in Kill(), but it might be convenient to (for example)
// turn off debug output from this instance.
//
func (kv *ShardKV) Kill() {
	atomic.StoreInt32(&kv.dead, 1)
	kv.rf.Kill()
}

func (kv *ShardKV) killed() bool {
	z := atomic.LoadInt32(&kv.dead)
	return z == 1
}

//
// servers[] contains the ports of the servers in this group.
//
// me is the index of the current server in servers[].
//
// the k/v server should store snapshots through the underlying Raft
// implementation, which should call persister.SaveStateAndSnapshot() to
// atomically save the Raft state along with the snapshot.
//
// the k/v server should snapshot when Raft's saved state exceeds
// maxraftstate bytes, in order to allow Raft to garbage-collect its
// log. if maxraftstate is -1, you don't need to snapshot.
//
// gid is this group's GID, for interacting with the shardmaster.
//
// pass masters[] to shardmaster.MakeClerk() so you can send
// RPCs to the shardmaster.
//
// make_end(servername) turns a server name from a
// Config.Groups[gid][i] into a labrpc.ClientEnd on which you can
// send RPCs. You'll need this to send RPCs to other groups.
//
// look at client.go for examples of how to use masters[]
// and make_end() to send RPCs to the group owning a specific shard.
//
// StartServer() must return quickly, so it should start goroutines
// for any long-running work.
//
func StartServer(servers []*labrpc.ClientEnd, me int, persister *raft.Persister, maxRaftState int, gid int, masters []*labrpc.ClientEnd, make_end func(string) *labrpc.ClientEnd) *ShardKV {
	// call labgob.Register on structures you want
	// Go's RPC library to marshall/unmarshall.
	labgob.Register(Command{})
	labgob.Register(CommandRequest{})
	labgob.Register(shardmaster.Config{})
	labgob.Register(ShardOperationResponse{})
	labgob.Register(ShardOperationRequest{})

	kv := new(ShardKV)
	kv.me = me
	kv.maxRaftState = maxRaftState
	kv.make_end = make_end
	kv.gid = gid
	kv.sc = shardmaster.MakeClerk(masters)

	// Your initialization code here.

	// Use something like this to talk to the shardmaster:
	// kv.mck = shardmaster.MakeClerk(kv.masters)

	kv.applyCh = make(chan raft.ApplyMsg)
	kv.rf = raft.Make(servers, me, persister, kv.applyCh)

	return kv
}
