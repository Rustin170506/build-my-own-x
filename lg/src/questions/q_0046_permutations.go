package questions

func permute(nums []int) [][]int {
	result := make([][]int, 0)
	current := make([]int, 0)
	visited := make([]bool, len(nums))

	var backtrace func()
	backtrace = func() {
		if len(current) == len(nums) {
			temp := make([]int, len(current))
			copy(temp, current)
			result = append(result, temp)
			return
		}

		for i := 0; i < len(nums); i++ {
			if visited[i] {
				continue
			}

			visited[i] = true
			current = append(current, nums[i])
			backtrace()
			current = current[:len(current)-1]
			visited[i] = false
		}
	}
	backtrace()
	return result
}
