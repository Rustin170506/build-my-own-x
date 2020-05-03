package raft

//
// this is an outline of the API that raft must expose to
// the service (or tester). see comments below for
// each of these functions for more details.
//
// rf = Make(...)
//   create a new Raft server.
// rf.Start(command interface{}) (index, term, isleader)
//   start agreement on a new log entry
// rf.GetState() (term, isLeader)
//   ask a Raft for its current term, and whether it thinks it is leader
// ApplyMsg
//   each time a new entry is committed to the log, each Raft peer
//   should send an ApplyMsg to the service (or tester)
//   in the same server.
//

import (
	"bytes"
	"math/rand"
	"sync"
	"time"
)
import "sync/atomic"
import "../labrpc"
import "../labgob"

//
// A Go object implementing a single Raft peer.
//
type Raft struct {
	mu        sync.Mutex          // Lock to protect shared access to this peer's state
	peers     []*labrpc.ClientEnd // RPC end points of all peers
	persister *Persister          // Object to hold this peer's persisted state
	me        int                 // this peer's index into peers[]
	dead      int32               // set by Kill()

	// Persistent state on all servers.
	leaderId        int         // Leader's id.
	currentTerm     int         // Lastest term server has seen, initialized to 0.
	votedFor        int         // Candidate that recevied in current term.
	state           serverState // Server state.
	lastReceiveTime time.Time   // Last get RPC call time.
	log             []LogEntry  // Log entries

	// Volatile state on all servers:
	commitIndex      int // Index of highest log entry known to be committed(initialized to 0, increases monotonically).
	lastAppliedIndex int // Index of highest log entry applied to state machine(initialized to 0, increases monotonically).

	// Volatile state on leaders:
	nextIndexes    []int // For each server, index of the next log entry to send to that server(initialized to leader last log index +1 ).
	matchedIndexes []int // For each server, index of highest log entry to be replicated on server(initialized to 0, increases monotonically).

	applyCh chan ApplyMsg // Is a channel on which the tester or service expects Raft to send ApplyMsg messages
}

// return currentTerm and whether this server
// believes it is the leader.
func (rf *Raft) GetState() (int, bool) {
	var term int
	var isLeader bool
	rf.mu.Lock()
	defer rf.mu.Unlock()
	term = rf.currentTerm
	isLeader = rf.state == Leader
	return term, isLeader
}

//
// save Raft's persistent state to stable storage,
// where it can later be retrieved after a crash and restart.
// see paper's Figure 2 for a description of what should be persistent.
//
func (rf *Raft) persist() {
	buffer := new(bytes.Buffer)
	encoder := labgob.NewEncoder(buffer)

	err := encoder.Encode(rf.currentTerm)
	if err != nil {
		DPrintf("%v encode currentTerm error: %v", rf.me, err)
	}

	err = encoder.Encode(rf.votedFor)
	if err != nil {
		DPrintf("%v encode votedFor error: %v", rf.me, err)
	}

	err = encoder.Encode(rf.log)
	if err != nil {
		DPrintf("%v encode log error: %v", rf.me, err)
	}

	data := buffer.Bytes()
	rf.persister.SaveRaftState(data)
}

//
// restore previously persisted state.
//
func (rf *Raft) readPersist(data []byte) {
	if data == nil || len(data) < 1 { // bootstrap without any state?
		return
	}
	r := bytes.NewBuffer(data)
	d := labgob.NewDecoder(r)
	currentTerm, votedFor := 0, 0
	err := d.Decode(&currentTerm)
	if err != nil {
		DPrintf("%v decode currentTerm error: %v", rf.me, err)
	}

	err = d.Decode(&votedFor)
	if err != nil {
		DPrintf("%v decode votedFor error: %v", rf.me, err)
	}

	err = d.Decode(&rf.log)
	if err != nil {
		DPrintf("%v decode log error: %v", rf.me, err)
	}

	rf.currentTerm, rf.votedFor = currentTerm, votedFor
}

//
// the service using Raft (e.g. a k/v server) wants to start
// agreement on the next command to be appended to Raft's log. if this
// server isn't the leader, returns false. otherwise start the
// agreement and return immediately. there is no guarantee that this
// command will ever be committed to the Raft log, since the leader
// may fail or lose an election. even if the Raft instance has been killed,
// this function should return gracefully.
//
// the first return value is the index that the command will appear at
// if it's ever committed. the second return value is the current
// term. the third return value is true if this server believes it is
// the leader.
//
func (rf *Raft) Start(command interface{}) (int, int, bool) {
	// The init state.
	index := -1
	term := -1
	isLeader := true
	// Get current state.
	term, isLeader = rf.GetState()
	rf.mu.Lock()
	defer rf.mu.Unlock()
	if isLeader {
		index = rf.getLastLogEntry().Index + 1
		entry := LogEntry{
			Command: command,
			Term:    term,
			Index:   index,
		}
		DPrintf("%v add command %v to self", rf.me, entry.Command)
		rf.log = append(rf.log, entry)
		// Update ourselves next indexes and matched indexes after we add a new log.
		rf.nextIndexes[rf.me] = len(rf.log)
		rf.matchedIndexes[rf.me] = len(rf.log) - 1
		rf.persist()
	}
	return index, term, isLeader
}

//
// the tester calls Kill() when a Raft instance won't
// be needed again. for your convenience, we supply
// code to set rf.dead (without needing a lock),
// and a killed() method to test rf.dead in
// long-running loops. you can also add your own
// code to Kill(). you're not required to do anything
// about this, but it may be convenient (for example)
// to suppress debug output from a Kill()ed instance.
//
func (rf *Raft) Kill() {
	atomic.StoreInt32(&rf.dead, 1)
	// Your code here, if desired.
}

// Is the server killed.
func (rf *Raft) killed() bool {
	z := atomic.LoadInt32(&rf.dead)
	return z == 1
}

// Is more up to date.
func (l LogEntry) isMoreUpToDate(r LogEntry) bool {
	return (l.Term == r.Term && l.Index >= r.Index) || l.Term > r.Term
}

// Start leader election.
// Kick off new election when election time out.
func (rf *Raft) startLeaderElection() {
	for {
		electionTimeout := rand.Intn(300)
		startTime := time.Now()
		time.Sleep(time.Duration(HeartbeatInterval*4+electionTimeout) * time.Millisecond)
		rf.mu.Lock()
		if atomic.LoadInt32(&rf.dead) == Dead {
			rf.mu.Unlock()
			return
		}
		if rf.lastReceiveTime.Before(startTime) {
			if rf.state != Leader {
				DPrintf("%d kicks off election on term: %d", rf.me, rf.currentTerm)
				go rf.kickOffElection()
			}
		}
		rf.mu.Unlock()
	}
}

// Kick off election to get new leader.
func (rf *Raft) kickOffElection() {
	rf.mu.Lock()
	rf.convertToCandidate()
	rf.persist()
	lastLogEntry := rf.getLastLogEntry()
	args := RequestVoteArgs{
		Term:         rf.currentTerm,
		CandidateId:  rf.me,
		LastLogIndex: lastLogEntry.Index,
		LastLogTerm:  lastLogEntry.Term,
	}
	numVote := 1
	rf.mu.Unlock()
	for i := 0; i < len(rf.peers); i++ {
		if i != rf.me {
			go func(peerId int) {
				reply := RequestVoteReply{}
				DPrintf("%d send vote request to %d", rf.me, peerId)
				ok := rf.sendRequestVoteRPC(peerId, &args, &reply)
				if !ok {
					return
				}
				rf.mu.Lock()
				defer rf.mu.Unlock()
				if reply.Term > rf.currentTerm {
					rf.convertToFollower(reply.Term)
					return
				}
				if reply.VoteGranted {
					numVote++
					// Get the most vote, so we can set myself as leader and start sync log.
					if numVote > len(rf.peers)/2 && rf.state == Candidate {
						rf.convertToLeader()
						DPrintf("%d become the leader on term: %d", rf.me, rf.currentTerm)
						for j := 0; j < len(rf.peers); j++ {
							if j != rf.me {
								go rf.replicaLogToPeer(j)
							}
						}
					}
				}
			}(i)
		}
	}
}

// Sync the log to peer node.
func (rf *Raft) replicaLogToPeer(peerId int) {
	DPrintf("%d start sync log to %d", rf.me, peerId)
	for {
		rf.mu.Lock()
		if rf.state != Leader {
			rf.mu.Unlock()
			DPrintf("%d stop sends append entries to %d", rf.me, peerId)
			return
		}
		rf.mu.Unlock()
		go rf.sendAppendEntry(peerId)
		time.Sleep(HeartbeatInterval * time.Millisecond)
	}
}

// Send append entry to other peers.
func (rf *Raft) sendAppendEntry(peerId int) {
	rf.mu.Lock()
	peerNextIndex := rf.nextIndexes[peerId]
	prevLogIndex := peerNextIndex - 1
	args := AppendEntriesArgs{
		Term:         rf.currentTerm,
		LeaderId:     rf.me,
		PrevLogIndex: prevLogIndex,
		PrevLogTerm:  rf.log[prevLogIndex].Term,
		Entries:      rf.log[rf.nextIndexes[peerId]:], // Log after next index.
		LeaderCommit: rf.commitIndex,
	}
	rf.mu.Unlock()
	reply := AppendEntriesReply{}
	// Because append entry handler AppendEntries also acquire the lock, so we need release this lock before send RPC.
	ok := rf.sendAppendEntryRPC(peerId, &args, &reply)
	if !ok {
		DPrintf("%d send a append PRC to %d failed", rf.me, peerId)
		return
	}
	rf.mu.Lock()
	if rf.state != Leader || rf.currentTerm != args.Term {
		rf.mu.Unlock()
		return
	}
	rf.mu.Unlock()

	rf.mu.Lock()
	defer rf.mu.Unlock()
	if reply.Success {
		// Update matched indexes and next indexes.
		rf.matchedIndexes[peerId] = args.PrevLogIndex + len(args.Entries)
		rf.nextIndexes[peerId] = rf.matchedIndexes[peerId] + 1
		DPrintf("%d success send a append to %d", rf.me, peerId)
		rf.updateCommittedIndex(rf.matchedIndexes)
		return
	} else {
		// If reply term more than current term, we should convert ourselves be a follower.
		if reply.Term > rf.currentTerm {
			rf.convertToFollower(reply.Term)
			return
		} else {
			// It means follower's log conflict with leader log.
			// So we need to fast back up.
			prevIndex := args.PrevLogIndex // Get the previous log index.

			// We will back up to a index which is first index of previous log term.
			for prevIndex > 0 && rf.log[prevIndex].Term == args.PrevLogTerm {
				prevIndex--
			}
			rf.nextIndexes[peerId] = prevIndex + 1
		}
	}
}

// Get the last log entry.
func (rf *Raft) getLastLogEntry() LogEntry {
	if len(rf.log) > 0 {
		return rf.log[len(rf.log)-1]
	} else {
		return LogEntry{nil, -1, -1}
	}
}

// Update committed index.
func (rf *Raft) updateCommittedIndex(matchedIndexes []int) {
	majorityIndex := getMajoritySameIndex(matchedIndexes)
	if rf.log[majorityIndex].Term == rf.currentTerm && majorityIndex > rf.commitIndex {
		rf.commitIndex = majorityIndex
		DPrintf("%v update commit index to %v", rf.me, rf.commitIndex)
	}
}

// Start apply message.
func (rf *Raft) startApply() {
	for {
		time.Sleep(5 * time.Millisecond)
		rf.mu.Lock()
		for rf.lastAppliedIndex < rf.commitIndex {
			rf.lastAppliedIndex++
			rf.persist()
			DPrintf("%v apply command %v", rf.me, rf.log[rf.lastAppliedIndex].Command)
			rf.applyCh <- ApplyMsg{CommandValid: true, CommandIndex: rf.lastAppliedIndex, Command: rf.log[rf.lastAppliedIndex].Command}
		}
		rf.mu.Unlock()
	}
}

//
// the service or tester wants to create a Raft server. the ports
// of all the Raft servers (including this one) are in peers[]. this
// server's port is peers[me]. all the servers' peers[] arrays
// have the same order. persister is a place for this server to
// save its persistent state, and also initially holds the most
// recent saved state, if any. applyCh is a channel on which the
// tester or service expects Raft to send ApplyMsg messages.
// Make() must return quickly, so it should start goroutines
// for any long-running work.
//
func Make(peers []*labrpc.ClientEnd, me int,
	persister *Persister, applyCh chan ApplyMsg) *Raft {
	rf := &Raft{}
	rf.peers = peers
	rf.persister = persister
	rf.me = me
	// Means no leader.
	rf.leaderId = -1
	rf.currentTerm = 0
	// Means not voted.
	rf.votedFor = -1
	rf.commitIndex = 0
	rf.lastAppliedIndex = 0
	rf.state = Follower
	rf.applyCh = applyCh
	rf.log = []LogEntry{{nil, 0, 0}} // log entry at index 0 is unused
	rf.matchedIndexes = make([]int, len(peers))
	rf.nextIndexes = make([]int, len(peers))
	rf.lastReceiveTime = time.Now()
	// Your initialization code here (2C).
	// Start leader election.
	go rf.startLeaderElection()
	// Start apply message.
	go rf.startApply()
	// initialize from state persisted before a crash
	rf.readPersist(persister.ReadRaftState())
	return rf
}
