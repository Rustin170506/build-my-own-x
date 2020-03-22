package raft

import "time"

//
// RequestVote RPC handler.
//
func (rf *Raft) RequestVote(args *RequestVoteArgs, reply *RequestVoteReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	DPrintf("%d start process vote request", rf.me)
	rf.lastReceiveTime = time.Now()

	lastEntry := rf.getLastLogEntry()

	logUpToDate := LogEntry{Command: nil, Index: args.LastLogIndex, Term: args.LastLogTerm}.isMoreUpToDate(lastEntry)

	reply.Term = rf.currentTerm
	if args.Term < rf.currentTerm {
		reply.VoteGranted = false
		DPrintf("%d refuse vote for %d", rf.me, args.CandidateId)
	} else {
		if args.Term > rf.currentTerm {
			rf.convertToFollower(args.Term)
		}
		if (rf.votedFor == -1 || rf.votedFor == args.CandidateId) && logUpToDate {
			rf.votedFor = args.CandidateId
			reply.VoteGranted = true
			DPrintf("%d vote for %d", rf.me, args.CandidateId)
		}
	}
	// Your code here (2B).
}

//
// AppendEntries RPC handler.
//
func (rf *Raft) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) {
	rf.mu.Lock()
	defer rf.mu.Unlock()
	DPrintf("%d start process append entries request from %d, args term: %d node term: %d", rf.me, args.LeaderId, args.Term, rf.currentTerm)
	if args.Term < rf.currentTerm {
		DPrintf("get append entries form %d, but the term less than %d", args.LeaderId, rf.me)
		reply.Term, reply.Success = rf.currentTerm, false
		return
	}
	reply.Term = rf.currentTerm
	rf.leaderId = args.LeaderId
	rf.lastReceiveTime = time.Now()

	if args.Term > rf.currentTerm {
		DPrintf("%d convert itself as follower", rf.me)
		rf.convertToFollower(args.Term)
	}
	// Your code here (2B).
}
