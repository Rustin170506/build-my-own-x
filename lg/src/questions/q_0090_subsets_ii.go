package questions

import "sort"

func subsetsWithDup(nums []int) [][]int {
	result := make([][]int, 0, len(nums))
	subset := make([]int, 0, len(nums))
	sort.Ints(nums)

	var backtrace func(index int)
	backtrace = func(index int) {
		if index == len(nums) {
			newSet := make([]int, len(subset))
			copy(newSet, subset)
			result = append(result, newSet)
			return
		}

		subset = append(subset, nums[index])
		backtrace(index + 1)
		subset = subset[:len(subset)-1]
		i := index
		for i+1 < len(nums) && nums[i] == nums[i+1] {
			i++
		}
		backtrace(i + 1)
	}
	backtrace(0)
	return result
}
