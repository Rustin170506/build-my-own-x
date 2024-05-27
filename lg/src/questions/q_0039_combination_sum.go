package questions

func combinationSum(candidates []int, target int) [][]int {
	result := make([][]int, 0)
	current := make([]int, 0)

	var backtrace func(index, total int)
	backtrace = func(index, total int) {
		if total == target {
			temp := make([]int, len(current))
			copy(temp, current)
			result = append(result, temp)
			return
		}

		if index >= len(candidates) || total > target {
			return
		}

		current = append(current, candidates[index])
		backtrace(index, total+candidates[index])
		current = current[:len(current)-1]
		backtrace(index+1, total)
	}
	backtrace(0, 0)
	return result
}
