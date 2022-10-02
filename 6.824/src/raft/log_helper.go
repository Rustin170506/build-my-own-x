package raft

// ======== The whole log entries ========
//
// Get the last log entry index.
func (rf *Raft) getLastLogEntryIndex() int {
	return rf.lastIncludedIndex + len(rf.log) - 1
}

// Get the log length.
func (rf *Raft) getLogLen() int {
	return len(rf.log) + rf.lastIncludedIndex
}

// Get the original log index. It represents the index in the whole log entries.
func (rf *Raft) getOriginalIndex(index int) int {
	return index + rf.lastIncludedIndex
}

// ======== The log entries after lastIncludedIndex ========
//
// Get the last log entry.
func (rf *Raft) getLastLogEntry() LogEntry {
	return rf.log[len(rf.log)-1]
}

// Get the log index with offset. It is a relative index in the current log entries.
func (rf *Raft) getLogIndexWithOffset(index int) int {
	DPrintf("[%d] get log index with offset, index: %d, lastIncludedIndex: %d", rf.me, index, rf.lastIncludedIndex)
	return index - rf.lastIncludedIndex
}

// Get log term by index and with offset.
func (rf *Raft) getLogTermWithOffset(index int) int {
	return rf.log[rf.getLogIndexWithOffset(index)].Term
}
