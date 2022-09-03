package raft

import (
	"time"
)

//
// RequestVote RPC handler.
//
func (rf *Raft) RequestVote(args *RequestVoteArgs, reply *RequestVoteReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	DPrintf("%d start process vote request", rf.me)
	rf.lastReceiveTime = time.Now()

	lastEntry := rf.getLastLogEntry()
	// To check dose the candidate got a longer logs.
	logUpToDate := LogEntry{Command: nil, Index: args.LastLogIndex, Term: args.LastLogTerm}.isMoreUpToDate(lastEntry)

	reply.Term = rf.currentTerm
	// If we get a term less than ourselves, we immediately refuse to vote for this candidate.
	if args.Term < rf.currentTerm {
		reply.VoteGranted = false
		DPrintf("%d refuse vote for %d", rf.me, args.CandidateId)
	} else {
		// If candidate term more than ourselves, we need to convert ourselves to be a follower.
		if args.Term > rf.currentTerm {
			rf.convertToFollower(args.Term)
		}
		// If we did not vote and the candidate got a longer logs, we need vote for this candidate.
		if rf.votedFor == -1 && logUpToDate {
			rf.votedFor = args.CandidateId
			reply.VoteGranted = true
			rf.persist()
			DPrintf("%d vote for %d", rf.me, args.CandidateId)
		}
	}
}

//
// AppendEntries RPC handler.
//
func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	DPrintf("[%d] start process append entries request from %d, args term: %d peer term: %d", rf.me, args.LeaderId, args.Term, rf.currentTerm)

	reply.Term = rf.currentTerm

	// If we got a term less than ourselves term, we need reject the append request.
	if args.Term < rf.currentTerm {
		DPrintf("get append entries form %d, but the term less than [%d]", args.LeaderId, rf.me)
		reply.Success = false
		return
	}

	// We need reset the timer for leader election, because we got a valid heartbeat or append entries request.
	rf.lastReceiveTime = time.Now()
	rf.leaderId = args.LeaderId

	// If we get a term more than ourselves term, we need covert ourselves be a follower.
	if args.Term > rf.currentTerm {
		DPrintf("[%d] convert itself as follower", rf.me)
		rf.convertToFollower(args.Term)
	}

	if args.PrevLogTerm < rf.lastIncludedIndex {
		reply.Success = false
		return
	}

	// Log got a conflict with leader log.
	if args.PrevLogIndex >= rf.getLogLen() || rf.getLogTermWithOffset(args.PrevLogIndex) != args.PrevLogTerm {
		// If our log length longer than leader log, we need to delete the useless log.
		if args.PrevLogIndex < rf.getLogLen() {
			rf.log = rf.log[0:rf.getLogIndexWithOffset(args.PrevLogIndex)] // Delete the log in prevLogIndex and after it.
			rf.persist()
		}
		reply.Success = false
		return
	}
	// If no conflict with leader's log.
	// We just append the entries to our log.
	rf.log = append(rf.log[0:rf.getLogIndexWithOffset(args.PrevLogIndex+1)], args.Entries...)
	rf.persist()
	// Update the committed index.
	if args.LeaderCommit > rf.commitIndex {
		rf.commitIndex = min(args.LeaderCommit, rf.getLastLogEntryIndex())
	}
	reply.Success = true
}

//
// InstallSnapshot RPC handler.
//
func (rf *Raft) InstallSnapshot(args *InstallSnapshotArgs, reply *InstallSnapshotReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	DPrintf("[%d] start process install snapshot request from %d, args term: %d peer term: %d", rf.me, args.LeaderId, args.Term, rf.currentTerm)

	reply.Term = rf.currentTerm
	// If we got a term less than ourselves term, we need reject the append request.
	if args.Term < rf.currentTerm {
		DPrintf("get install snapshot form %d, but the term less than [%d]", args.LeaderId, rf.me)
		reply.Success = false
		return
	}

	// We need reset the timer for leader election, because we got a valid heartbeat or append entries request.
	rf.lastReceiveTime = time.Now()
	rf.leaderId = args.LeaderId

	// If we get a term more than ourselves term, we need covert ourselves be a follower.
	if args.Term > rf.currentTerm {
		DPrintf("[%d] convert itself as follower", rf.me)
		rf.convertToFollower(args.Term)
	}

	// If we already have this snapshot, we just return.
	if args.LastIncludedIndex <= rf.lastIncludedIndex {
		reply.Success = false
		return
	}

	// If we have more log than the snapshot, we need to delete the useless log.
	if args.LastIncludedIndex < rf.getLogLen() {
		// If in the same term, we need to delete the log after the snapshot.
		if rf.getLogTermWithOffset(args.LastIncludedIndex) == args.LastIncludedTerm {
			rf.log = append(make([]LogEntry, 0), rf.log[args.LastIncludedIndex-rf.lastIncludedIndex:]...)
		} else {
			// If in different term, we need to delete all the log.
			rf.log = make([]LogEntry, 0)
		}
	} else {
		// If the snapshot is more than our log, we need to create a new log and drop the old log.
		rf.log = make([]LogEntry, 0)
	}

	// Persist the snapshot.
	rf.lastIncludedIndex = args.LastIncludedIndex
	rf.lastIncludedTerm = args.LastIncludedTerm
	rf.persistStateAndSnapshot(args.Snapshot)

	DPrintf("[%d] install snapshot from %d, lastIncludedIndex: %d, lastIncludedTerm: %d, lastAppliedIndex: %d", rf.me, args.LeaderId, args.LastIncludedIndex, args.LastIncludedTerm, rf.lastAppliedIndex)
	// If needed, we apply the snapshot.
	if rf.lastIncludedIndex >= rf.lastAppliedIndex {
		rf.lastAppliedIndex = rf.lastIncludedIndex
		msg := ApplyMsg{Snapshot: args.Snapshot, CommandValid: false, CommandIndex: rf.lastAppliedIndex}
		rf.applyCh <- msg
		DPrintf("[%d] send snapshot to server", rf.me)
	}
	reply.Success = true
}
