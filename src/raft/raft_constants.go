package raft

// The raft server state, default is Follower.
const (
	Leader serverState = iota
	Candidate
	Follower
)

// Is the server dead.
const Dead int32 = 1

// Heartbeat interval.
const HeartbeatInterval = 100
