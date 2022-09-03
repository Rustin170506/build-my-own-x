package raft

import "bytes"
import "../labgob"

//
// save Raft's persistent state to stable storage,
// where it can later be retrieved after a crash and restart.
// see paper's Figure 2 for a description of what should be persistent.
//
func (rf *Raft) persist() {
	state := rf.encodeRaftState()
	rf.persister.SaveRaftState(state)
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
	err := d.Decode(&rf.currentTerm)
	if err != nil {
		DPrintf("[%d] decode currentTerm error: %v", rf.me, err)
	}

	err = d.Decode(&rf.votedFor)
	if err != nil {
		DPrintf("[%d] decode votedFor error: %v", rf.me, err)
	}

	err = d.Decode(&rf.log)
	if err != nil {
		DPrintf("[%d] decode log error: %v", rf.me, err)
	}

	err = d.Decode(&rf.lastIncludedIndex)
	if err != nil {
		DPrintf("[%d] decode lastIncludedIndex error: %v", rf.me, err)
	}

	err = d.Decode(&rf.lastIncludedTerm)
	if err != nil {
		DPrintf("[%d] decode lastIncludedTerm error: %v", rf.me, err)
	}
}

// persistStateAndSnapshot persists the snapshot to the persister.
func (rf *Raft) persistStateAndSnapshot(snapshot []byte) {
	state := rf.encodeRaftState()
	rf.persister.SaveStateAndSnapshot(state, snapshot)
}

// encodeRaftState encodes the raft state into a byte array.
func (rf *Raft) encodeRaftState() []byte {
	buffer := new(bytes.Buffer)
	encoder := labgob.NewEncoder(buffer)

	err := encoder.Encode(rf.currentTerm)
	if err != nil {
		DPrintf("[%d] encode currentTerm error: %v", rf.me, err)
	}

	err = encoder.Encode(rf.votedFor)
	if err != nil {
		DPrintf("[%d] encode votedFor error: %v", rf.me, err)
	}

	err = encoder.Encode(rf.log)
	if err != nil {
		DPrintf("[%d] encode log error: %v", rf.me, err)
	}

	err = encoder.Encode(rf.lastIncludedIndex)
	if err != nil {
		DPrintf("[%d] encode lastIncludedIndex error: %v", rf.me, err)
	}

	err = encoder.Encode(rf.lastIncludedTerm)
	if err != nil {
		DPrintf("[%d] encode lastIncludedTerm error: %v", rf.me, err)
	}

	return buffer.Bytes()
}

// IsExceedMaxRaftState returns true if the raft state size exceeds maxRaftState.
func (rf *Raft) IsExceedMaxRaftState(maxRaftState int) bool {
	return rf.persister.RaftStateSize() >= maxRaftState
}
