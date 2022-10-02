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
	DPrintf("[%d] start process append entries request from %d, args term: %d peer term: %d args: %v", rf.me, args.LeaderId, args.Term, rf.currentTerm, args)

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

	if args.PrevLogIndex < rf.lastIncludedIndex {
		DPrintf("[%d] get append entries from %d, but the prev log term less than last included index", rf.me, args.LeaderId)
		reply.Success = true
		reply.ConflictIndex = rf.lastIncludedIndex + 1
		return
	}

	// Log got a conflict with leader log.
	if args.PrevLogIndex >= rf.getLogLen() || rf.getLogTermWithOffset(args.PrevLogIndex) != args.PrevLogTerm {
		// From big to small, find the first index that has the same term with leader.
		conflictIndex := min(rf.getLastLogEntryIndex(), args.PrevLogIndex)
		conflictTerm := rf.getLogTermWithOffset(conflictIndex)
		lowBound := max(rf.lastIncludedIndex, rf.commitIndex)
		for conflictIndex > lowBound && rf.getLogTermWithOffset(conflictIndex-1) == conflictTerm {
			conflictIndex--
		}
		reply.Success = false
		reply.ConflictIndex = conflictIndex
		return
	}
	// If no conflict with leader's log.
	// We just append the entries to our log.
	rf.log = append(rf.log[0:rf.getLogIndexWithOffset(args.PrevLogIndex+1)], args.Entries...)
	rf.persist()
	// Update the committed index.
	if args.LeaderCommit > rf.commitIndex {
		DPrintf("[%d] update commit index from %d to %d", rf.me, rf.commitIndex, args.LeaderCommit)
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
		return
	}

	newLog := []LogEntry{{nil, 0, 0}}
	// If the snapshot is more than our log, we need to create a new log and drop the old log.
	if args.LastIncludedIndex >= rf.getLastLogEntryIndex() {
		if rf.commitIndex < args.LastIncludedIndex {
			rf.commitIndex = args.LastIncludedIndex
		}
	} else if rf.getLogTermWithOffset(args.LastIncludedIndex) != args.LastIncludedTerm {
		rf.commitIndex = args.LastIncludedIndex
	} else {
		newLog = append(newLog, rf.log[rf.getLogIndexWithOffset(args.LastIncludedIndex)+1:]...)
		if rf.commitIndex < args.LastIncludedIndex {
			rf.commitIndex = args.LastIncludedIndex
		}
	}

	// Persist the snapshot.
	rf.log = newLog
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
}
