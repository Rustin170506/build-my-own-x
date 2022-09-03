package kvraft

import (
	"bytes"
	"log"
	"sync"
	"sync/atomic"
	"time"

	"../labgob"
	"../labrpc"
	"../raft"
)

const (
	GetOp    = "Get"
	PutOp    = "Put"
	AppendOp = "Append"
)

const (
	// timeout is the timeout for an op.
	timeout = 100 * time.Millisecond
	// compactInterval is the interval for compact.
	compactInterval = 1 * time.Millisecond
)

const Debug = 0

func DPrintf(format string, a ...interface{}) (n int, err error) {
	if Debug > 0 {
		log.Printf(format, a...)
	}
	return
}

type Op struct {
	Type      string
	Key       string
	Value     string
	ClientID  int64
	RequestID int64
}

type KVServer struct {
	mu      sync.Mutex
	me      int
	rf      *raft.Raft
	applyCh chan raft.ApplyMsg
	dead    int32 // set by Kill()

	db               map[string]string // KV database
	opMap            map[int]chan Op   // CommandIndex -> Channel
	lastRequestIDMap map[int64]int64   // ClientID -> RequestID

	maxRaftState     int // snapshot if log grows this big
	lastAppliedIndex int // last index that has been applied
}

func (kv *KVServer) Get(args *GetArgs, reply *GetReply) {
	op := Op{
		Type:      GetOp,
		Key:       args.Key,
		Value:     "",
		ClientID:  args.ClientID,
		RequestID: args.RequestID,
	}
	isLeader := kv.startOp(op, timeout)
	if !isLeader {
		reply.Err = ErrWrongLeader
		return
	}
	kv.mu.Lock()
	reply.Value = kv.db[args.Key]
	kv.mu.Unlock()
	DPrintf("[%d] got key: %s, value: %s", kv.me, args.Key, reply.Value)
	reply.Err = OK
}

func (kv *KVServer) PutAppend(args *PutAppendArgs, reply *PutAppendReply) {
	op := Op{
		Type:      args.Op,
		Key:       args.Key,
		Value:     args.Value,
		ClientID:  args.ClientID,
		RequestID: args.RequestID,
	}
	isLeader := kv.startOp(op, timeout)
	if !isLeader {
		DPrintf("[%d] putted/appended key: %s, value: %s failed", kv.me, args.Key, args.Value)
		reply.Err = ErrWrongLeader
		return
	}
	DPrintf("[%d] putted/appended key: %s, value: %s", kv.me, args.Key, args.Value)
	reply.Err = OK
}

func (kv *KVServer) startOp(op Op, timeout time.Duration) (isLeader bool) {
	kv.mu.Lock()
	index, _, isLeader := kv.rf.Start(op)
	if !isLeader {
		kv.mu.Unlock()
		return
	}
	if _, ok := kv.opMap[index]; !ok {
		DPrintf("[%d] create op channel, index: %d, op: %v", kv.me, index, op)
		kv.opMap[index] = make(chan Op, 1)
	}

	ch := kv.opMap[index]
	kv.mu.Unlock()

	select {
	case opRecv := <-ch:
		{
			kv.mu.Lock()
			delete(kv.opMap, index)
			DPrintf("[%d] drop the op channel, index: %d", kv.me, index)
			if opRecv.RequestID != op.RequestID || opRecv.ClientID != op.ClientID {
				DPrintf("[%d] receive not match opRecv: %v, op: %v", kv.me, opRecv, op)
				isLeader = false
			} else {
				kv.lastAppliedIndex = index
			}
			kv.mu.Unlock()
		}
	case <-time.After(timeout):
		DPrintf("[%d] timeout, index: %d, op: %v", kv.me, index, op)
		isLeader = false
	}

	return isLeader
}

func (kv *KVServer) receive() {
	for msg := range kv.applyCh {
		DPrintf("[%d] receive op...", kv.me)
		kv.mu.Lock()
		if !msg.CommandValid {
			DPrintf("[%d] receive snapshot, lastAppliedIndex: %d", kv.me, kv.lastAppliedIndex)
			kv.readSnapshot(msg.Snapshot)
			kv.lastAppliedIndex = msg.CommandIndex
			kv.mu.Unlock()
			continue
		}
		op := msg.Command.(Op)
		lastRequestID, ok := kv.lastRequestIDMap[op.ClientID]
		if !ok || lastRequestID < op.RequestID {
			DPrintf("[%d] update last request ID, Client: %d, Request: %d", kv.me, op.ClientID, op.RequestID)
			kv.lastRequestIDMap[op.ClientID] = op.RequestID
			switch op.Type {
			case PutOp:
				DPrintf("[%d] db put key: %s, value: %s", kv.me, op.Key, op.Value)
				kv.db[op.Key] = op.Value
			case AppendOp:
				DPrintf("[%d] db append key: %s, value: %s", kv.me, op.Key, op.Value)
				kv.db[op.Key] += op.Value
			case GetOp:
				// do nothing
			default:
				DPrintf("unknown type op: %s", op.Type)
			}
		}
		ch, ok := kv.opMap[msg.CommandIndex]
		kv.mu.Unlock()
		if ok {
			ch <- op
		} else {
			DPrintf("[%d] can not find op channel, index: %d", kv.me, msg.CommandIndex)
		}
		DPrintf("[%d] receive op done...", kv.me)
	}
}

// readSnapshot reads snapshot and recover the db and lastRequestIDMap.
func (kv *KVServer) readSnapshot(snapshot []byte) {
	DPrintf("[%d] try to read snapshot...", kv.me)
	if len(snapshot) == 0 {
		return
	}

	r := bytes.NewBuffer(snapshot)
	d := labgob.NewDecoder(r)
	var db map[string]string
	var lastRequestIDMap map[int64]int64

	err := d.Decode(&db)
	if err != nil {
		DPrintf("[%d] decode db error: %v", kv.me, err)
	}
	err = d.Decode(&lastRequestIDMap)
	if err != nil {
		DPrintf("[%d] decode lastRequestIDMap error: %v", kv.me, err)
	}

	DPrintf("[%d] read snapshot, db: %v, lastRequestIDMap: %v", kv.me, db, lastRequestIDMap)
	kv.db, kv.lastRequestIDMap = db, lastRequestIDMap
}

// compact the log and saves the snapshot.
func (kv *KVServer) compact() {
	for {
		if kv.killed() || kv.maxRaftState == -1 {
			return
		}
		if kv.rf.IsExceedMaxRaftState(kv.maxRaftState) {
			kv.mu.Lock()
			snapshot := kv.generateSnapshot()
			lastAppliedIndex := kv.lastAppliedIndex
			kv.mu.Unlock()
			if len(snapshot) > 0 {
				kv.rf.CompactLog(lastAppliedIndex, snapshot)
			}
		}
		// Avoid cpu spin.
		time.Sleep(compactInterval)
	}
}

// generateSnapshot generates the snapshot it includes the db and lastRequestIDMap.
func (kv *KVServer) generateSnapshot() []byte {
	w := new(bytes.Buffer)
	e := labgob.NewEncoder(w)
	err := e.Encode(kv.db)
	if err != nil {
		DPrintf("[%d] encode db error: %v", kv.me, err)
	}
	err = e.Encode(kv.lastRequestIDMap)
	if err != nil {
		DPrintf("[%d] encode lastRequestIDMap error: %v", kv.me, err)
	}

	return w.Bytes()
}

//
// the tester calls Kill() when a KVServer instance won't
// be needed again. for your convenience, we supply
// code to set rf.dead (without needing a lock),
// and a killed() method to test rf.dead in
// long-running loops. you can also add your own
// code to Kill(). you're not required to do anything
// about this, but it may be convenient (for example)
// to suppress debug output from a Kill()ed instance.
//
func (kv *KVServer) Kill() {
	atomic.StoreInt32(&kv.dead, 1)
	kv.rf.Kill()
}

func (kv *KVServer) killed() bool {
	z := atomic.LoadInt32(&kv.dead)
	return z == 1
}

//
// servers[] contains the ports of the set of
// servers that will cooperate via Raft to
// form the fault-tolerant key/value service.
// me is the index of the current server in servers[].
// the k/v server should store snapshots through the underlying Raft
// implementation, which should call persister.SaveStateAndSnapshot() to
// atomically save the Raft state along with the snapshot.
// the k/v server should snapshot when Raft's saved state exceeds maxRaftState bytes,
// in order to allow Raft to garbage-collect its log. if maxRaftState is -1,
// you don't need to snapshot.
// StartKVServer() must return quickly, so it should start goroutines
// for any long-running work.
//
func StartKVServer(servers []*labrpc.ClientEnd, me int, persister *raft.Persister, maxRaftState int) *KVServer {
	// call labgob.Register on structures you want
	// Go's RPC library to marshall/unmarshall.
	labgob.Register(Op{})

	kv := new(KVServer)
	kv.me = me
	kv.maxRaftState = maxRaftState

	kv.lastRequestIDMap = make(map[int64]int64)
	kv.db = make(map[string]string)
	kv.opMap = make(map[int]chan Op)

	kv.applyCh = make(chan raft.ApplyMsg)
	kv.rf = raft.Make(servers, me, persister, kv.applyCh)
	// Try to recover from snapshot.
	snapshot := persister.ReadSnapshot()
	kv.mu.Lock()
	kv.readSnapshot(snapshot)
	kv.mu.Unlock()

	go kv.receive()
	go kv.compact()

	return kv
}
