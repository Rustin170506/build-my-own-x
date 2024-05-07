package questions

import "slices"

func lengthOfLIS(nums []int) int {
	list := make([]int, len(nums))
	for i := range list {
		list[i] = 1
	}

	for i := len(nums); i >= 0; i-- {
		for j := i + 1; j < len(nums); j++ {
			if nums[i] < nums[j] {
				list[i] = max(list[i], 1+list[j])
			}
		}
	}
	return slices.Max(list)
}
