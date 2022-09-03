package raft

import (
	"log"
	"sort"
)

// Debugging
const Debug = 0

// Debug printf.
func DPrintf(format string, a ...interface{}) (n int, err error) {
	if Debug > 0 {
		log.Printf(format, a...)
	}
	return
}

// Get most peers matched index.
func getMajoritySameIndex(matchedIndexes []int) int {
	tmp := make([]int, len(matchedIndexes))
	copy(tmp, matchedIndexes)
	sort.Ints(tmp)
	idx := len(tmp) / 2
	return tmp[idx]
}

func min(x, y int) int {
	if x < y {
		return x
	}
	return y
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}
