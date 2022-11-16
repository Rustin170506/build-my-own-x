package shardmaster

//
// Shardmaster clerk.
//

import "../labrpc"
import "time"
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

func (ck *Clerk) Query(num int) Config {
	args := &QueryArgs{
		Num:       num,
		RequestID: ck.requestID,
		ClientID:  ck.clientID,
	}
	ck.requestID++
	for {
		var reply QueryReply

		if ck.callLeader("ShardMaster.Query", args, &reply) {
			if reply.Err == OK {
				return reply.Config
			}
		}
		ck.changeLeader()
		time.Sleep(retryInterval)
	}
}

func (ck *Clerk) Join(servers map[int][]string) {
	args := &JoinArgs{
		Servers:   servers,
		RequestID: ck.requestID,
		ClientID:  ck.clientID,
	}
	ck.requestID++

	for {
		var reply JoinReply

		if ck.callLeader("ShardMaster.Join", args, &reply) {
			if reply.Err == OK {
				return
			}
		}
		ck.changeLeader()
		time.Sleep(retryInterval)
	}
}

func (ck *Clerk) Leave(gids []int) {
	args := &LeaveArgs{
		GIDs:      gids,
		RequestID: ck.requestID,
		ClientID:  ck.clientID,
	}
	ck.requestID++

	// Your code here.
	args.GIDs = gids

	for {
		var reply LeaveReply

		if ck.callLeader("ShardMaster.Leave", args, &reply) {
			if reply.Err == OK {
				return
			}
		}
		ck.changeLeader()
		time.Sleep(retryInterval)
	}
}

func (ck *Clerk) Move(shard int, gid int) {
	args := &MoveArgs{
		Shard:     shard,
		GID:       gid,
		RequestID: ck.requestID,
		ClientID:  ck.clientID,
	}
	ck.requestID++

	for {
		var reply MoveReply

		if ck.callLeader("ShardMaster.Move", args, &reply) {
			if reply.Err == OK {
				return
			}
		}
		ck.changeLeader()
		time.Sleep(retryInterval)
	}
}

func (ck *Clerk) callLeader(rpcname string, args interface{}, reply interface{}) bool {
	return ck.servers[ck.leaderID].Call(rpcname, args, reply)
}

func (ck *Clerk) changeLeader() {
	ck.leaderID = (ck.leaderID + 1) % int64(len(ck.servers))
}
