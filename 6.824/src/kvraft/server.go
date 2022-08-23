package kvraft

import (
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

const Timeout = 100 * time.Millisecond

const Debug = 1

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
	lastrequestIDMap map[int64]int64   // ClientID -> RequestID

	maxraftstate int // snapshot if log grows this big
}

func (kv *KVServer) Get(args *GetArgs, reply *GetReply) {
	op := Op{
		Type:      GetOp,
		Key:       args.Key,
		Value:     "",
		ClientID:  args.ClientID,
		RequestID: args.RequestID,
	}
	isLeader := kv.startOp(op, Timeout)
	if !isLeader {
		reply.Err = ErrWrongLeader
		return
	}
	kv.mu.Lock()
	reply.Value = kv.db[args.Key]
	kv.mu.Unlock()
	DPrintf("got key: %s, value: %s", args.Key, reply.Value)
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
	isLeader := kv.startOp(op, Timeout)
	if !isLeader {
		reply.Err = ErrWrongLeader
		return
	}
	DPrintf("putted/appended key: %s, value: %s", args.Key, args.Value)
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
		DPrintf("create op channel, index: %d", index)
		kv.opMap[index] = make(chan Op, 1)
	}

	ch := kv.opMap[index]
	kv.mu.Unlock()

	select {
	case opRecv := <-ch:
		{
			kv.mu.Lock()
			delete(kv.opMap, index)
			DPrintf("drop the op channel, index: %d", index)
			if opRecv.RequestID != op.RequestID || opRecv.ClientID != op.ClientID {
				isLeader = false
			}
			kv.mu.Unlock()
		}
	case <-time.After(timeout):
		isLeader = false
	}

	return isLeader
}

func (kv *KVServer) receive() {
	for msg := range kv.applyCh {
		DPrintf("receive op...")
		if !msg.CommandValid {
			continue
		}
		kv.mu.Lock()
		op := msg.Command.(Op)
		lastRequestID, ok := kv.lastrequestIDMap[op.ClientID]
		if !ok || lastRequestID < op.RequestID {
			DPrintf("update last request ID, Client: %d, Request: %d", op.ClientID, op.RequestID)
			kv.lastrequestIDMap[op.ClientID] = op.RequestID
			switch op.Type {
			case PutOp:
				DPrintf("db put key: %s, value: %s", op.Key, op.Value)
				kv.db[op.Key] = op.Value
			case AppendOp:
				DPrintf("db append key: %s, value: %s", op.Key, op.Value)
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
			DPrintf("can not find op channel, index: %d", msg.CommandIndex)
		}
		DPrintf("receive op done...")
	}
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
// the k/v server should snapshot when Raft's saved state exceeds maxraftstate bytes,
// in order to allow Raft to garbage-collect its log. if maxraftstate is -1,
// you don't need to snapshot.
// StartKVServer() must return quickly, so it should start goroutines
// for any long-running work.
//
func StartKVServer(servers []*labrpc.ClientEnd, me int, persister *raft.Persister, maxraftstate int) *KVServer {
	// call labgob.Register on structures you want
	// Go's RPC library to marshall/unmarshall.
	labgob.Register(Op{})

	kv := new(KVServer)
	kv.me = me
	kv.maxraftstate = maxraftstate

	kv.lastrequestIDMap = make(map[int64]int64)
	kv.db = make(map[string]string)
	kv.opMap = make(map[int]chan Op)

	// FIXME: It shouldn't be a buffer channel.
	kv.applyCh = make(chan raft.ApplyMsg, 1024)
	kv.rf = raft.Make(servers, me, persister, kv.applyCh)

	go kv.receive()

	return kv
}
