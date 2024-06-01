package questions

import "sort"

func combinationSum2(candidates []int, target int) [][]int {
	result := make([][]int, 0)
	sort.Ints(candidates)
	current := make([]int, 0)
	var backtrack func(index, newTarget int)
	backtrack = func(index, newTarget int) {
		if newTarget == 0 {
			newTemp := make([]int, len(current))
			copy(newTemp, current)
			result = append(result, newTemp)
			return
		}
		if newTarget < 0 {
			return
		}

		pos := index
		prev := -1
		for pos < len(candidates) {
			if candidates[pos] == prev {
				pos++
				continue
			}
			current = append(current, candidates[pos])
			backtrack(pos+1, newTarget-candidates[pos])
			current = current[:len(current)-1]
			prev = candidates[pos]
			pos++
		}
	}

	backtrack(0, target)
	return result
}
