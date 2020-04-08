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
	DPrintf("%d start process append entries request from %d, args term: %d peer term: %d", rf.me, args.LeaderId, args.Term, rf.currentTerm)

	reply.Term = rf.currentTerm

	// If we got a term less than ourselves term, we need reject the append request.
	if args.Term < rf.currentTerm {
		DPrintf("get append entries form %d, but the term less than %d", args.LeaderId, rf.me)
		reply.Success = false
		return
	}

	// We need reset the timer for leader election, because we got a valid heartbeat or append entries request.
	rf.lastReceiveTime = time.Now()
	rf.leaderId = args.LeaderId

	// If we get a term more than ourselves term, we need covert ourselves be a follower.
	if args.Term > rf.currentTerm {
		DPrintf("%d convert itself as follower", rf.me)
		rf.convertToFollower(args.Term)
	}
	// Log got a conflict with leader log.
	if args.PrevLogIndex >= len(rf.log) || rf.log[args.PrevLogIndex].Term != args.PrevLogTerm {
		// If our log length longer than leader log, we need to delete the useless log.
		if args.PrevLogIndex < len(rf.log) {
			rf.log = rf.log[0:args.PrevLogIndex] // Delete the log in prevLogIndex and after it.
			// Your code here (2C).
		}
		reply.Success = false
		return
	}
	// If no conflict with leader's log.
	// We just append the entries to our log.
	rf.log = append(rf.log[0:args.PrevLogIndex+1], args.Entries...)
	// Update the committed index.
	if args.LeaderCommit > rf.commitIndex {
		rf.commitIndex = min(args.LeaderCommit, len(rf.log)-1)
	}
	reply.Success = true
}
