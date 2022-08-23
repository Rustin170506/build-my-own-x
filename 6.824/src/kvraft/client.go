package kvraft

import (
	"../labrpc"
	"time"
)
import "crypto/rand"
import "math/big"

// retryInterval is the interval between retries to send an RPC.
const retryInterval = time.Millisecond * 60

type Clerk struct {
	servers []*labrpc.ClientEnd
	// Cache the leaderID.
	leaderID int64
	// clientID is used to distinguish different clients.
	clientID int64
	// requestID is used to distinguish different requests.
	requestID int64
}

func nrand() int64 {
	max := big.NewInt(int64(1) << 62)
	bigx, _ := rand.Int(rand.Reader, max)
	x := bigx.Int64()
	return x
}

func MakeClerk(servers []*labrpc.ClientEnd) *Clerk {
	ck := new(Clerk)
	ck.servers = servers
	ck.clientID = nrand()
	ck.leaderID = 0
	ck.requestID = 0
	return ck
}

//
// fetch the current value for a key.
// returns "" if the key does not exist.
// keeps trying forever in the face of all other errors.
//
// you can send an RPC with code like this:
// ok := ck.servers[i].Call("KVServer.Get", &args, &reply)
//
// the types of args and reply (including whether they are pointers)
// must match the declared types of the RPC handler function's
// arguments. and reply must be passed as a pointer.
//
func (ck *Clerk) Get(key string) string {
	args := GetArgs{
		Key:       key,
		RequestID: ck.requestID,
		ClientID:  ck.clientID,
	}
	ck.requestID++
	DPrintf("%d get key: %s", ck.clientID, args.Key)
	for {
		var reply GetReply
		if ck.callLeader("KVServer.Get", &args, &reply) {
			if reply.Err == OK {
				return reply.Value
			}
			if reply.Err == ErrNoKey {
				return ""
			}
		}
		ck.changeLeader()
		time.Sleep(retryInterval)
	}
}

//
// shared by Put and Append.
//
// you can send an RPC with code like this:
// ok := ck.servers[i].Call("KVServer.PutAppend", &args, &reply)
//
// the types of args and reply (including whether they are pointers)
// must match the declared types of the RPC handler function's
// arguments. and reply must be passed as a pointer.
//
func (ck *Clerk) PutAppend(key string, value string, op string) {
	args := PutAppendArgs{Key: key, Value: value, Op: op, RequestID: ck.requestID, ClientID: ck.clientID}
	ck.requestID++
	DPrintf("%d put/append key: %s, value: %s", ck.clientID, args.Key, args.Value)
	for {
		var reply PutAppendReply
		if ck.callLeader("KVServer.PutAppend", &args, &reply) {
			if reply.Err == OK {
				break
			}
		}
		ck.changeLeader()
		time.Sleep(retryInterval)
	}
}

func (ck *Clerk) Put(key string, value string) {
	ck.PutAppend(key, value, PutOp)
}
func (ck *Clerk) Append(key string, value string) {
	ck.PutAppend(key, value, AppendOp)
}

func (ck *Clerk) callLeader(rpcname string, args interface{}, reply interface{}) bool {
	return ck.servers[ck.leaderID].Call(rpcname, args, reply)
}

func (ck *Clerk) changeLeader() {
	ck.leaderID = (ck.leaderID + 1) % int64(len(ck.servers))
}
