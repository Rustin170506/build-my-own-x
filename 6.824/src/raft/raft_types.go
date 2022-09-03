package raft

//
// example RequestVote RPC arguments structure.
// field names must start with capital letters!
//
type RequestVoteArgs struct {
	Term         int // Candidate's term.
	CandidateId  int // Candidate's id.
	LastLogIndex int // Candidate's latest log entry index.
	LastLogTerm  int // Candidate's latest log entry term.
}

//
// example RequestVote RPC reply structure.
// field names must start with capital letters!
//
type RequestVoteReply struct {
	Term        int  // Current term, for candidate to update itself.
	VoteGranted bool // true means candidate received vote.
}

// Log entry, used by Leader to sync log.
type LogEntry struct {
	Command interface{}
	Term    int // Term number when received.
	Index   int // The log index.
}

// Is more up to date.
func (l LogEntry) isMoreUpToDate(r LogEntry) bool {
	return (l.Term == r.Term && l.Index >= r.Index) || l.Term > r.Term
}

// Append entries RPC args.
// Invoked by leader to replicate log entries, also used as heartbeat.
type AppendEntriesArgs struct {
	Term         int        // Leader's term.
	LeaderId     int        // Leader's id.
	PrevLogIndex int        // Index of log entry immediately pre log index.
	PrevLogTerm  int        // prevLogIndex's term.
	Entries      []LogEntry // Log entries, for heartbeat is nil.
	LeaderCommit int        // Leader already committed index.
}

// Append RPC reply.
type AppendEntriesReply struct {
	Term    int  // Current term, for leader to update itself.
	Success bool // true if follower contained entry matching prevLogIndex and prevLogTerm.
}

// Install snapshot RPC args.
type InstallSnapshotArgs struct {
	Term              int    // Leader's term.
	LeaderId          int    // Leader's id.
	LastIncludedIndex int    // The snapshot replaces all entries up through and including this index.
	LastIncludedTerm  int    // The term of lastIncludedIndex.
	Snapshot          []byte // The actual snapshot data.
}

type InstallSnapshotReply struct {
	Term    int  // Current term, for leader to update itself.
	Success bool // true if follower install the snapshot.
}

//
// as each Raft peer becomes aware that successive log entries are
// committed, the peer should send an ApplyMsg to the service (or
// tester) on the same server, via the applyCh passed to Make(). set
// CommandValid to true to indicate that the ApplyMsg contains a newly
// committed log entry.
//
// in Lab 3 you'll want to send other kinds of messages (e.g.,
// snapshots) on the applyCh; at that point you can add fields to
// ApplyMsg, but set CommandValid to false for these other uses.
//
type ApplyMsg struct {
	CommandValid bool
	Command      interface{}
	CommandIndex int
	Snapshot     []byte
}

// Raft server state.
type serverState int32

// Get state string.
func (state serverState) String() string {
	switch state {
	case Leader:
		return "Leader"
	case Candidate:
		return "Candidate"
	default:
		return "Follower"
	}
}
