package raft

import "time"

// Convert to candidate and update lastReceiveTime.
func (rf *Raft) convertToCandidate() {
	rf.state = Candidate
	rf.currentTerm++
	rf.votedFor = rf.me
	rf.lastReceiveTime = time.Now()
}

// Convert to follower and update lastReceiveTime.
func (rf *Raft) convertToFollower(newTerm int) {
	rf.state = Follower
	rf.currentTerm = newTerm
	rf.votedFor = -1
	rf.lastReceiveTime = time.Now()
}

// Convert to leader and update lastReceiveTime.
func (rf *Raft) convertToLeader() {
	rf.state = Leader
	rf.lastReceiveTime = time.Now()
}
