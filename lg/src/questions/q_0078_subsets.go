package questions

func subsets(nums []int) [][]int {
	result := make([][]int, 0)

	subset := make([]int, 0, len(nums))
	var dfs func(index int)
	dfs = func(index int) {
		if index >= len(nums) {
			s := make([]int, len(subset))
			copy(s, subset)
			result = append(result, s)
			return
		}
		subset = append(subset, nums[index])
		dfs(index + 1)
		subset = subset[:len(subset)-1]
		dfs(index + 1)
	}

	dfs(0)

	return result
}
