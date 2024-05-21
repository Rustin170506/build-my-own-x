package questions

import "slices"

func threeSum(nums []int) [][]int {
	slices.Sort(nums)
	result := make([][]int, 0)

	for i := 0; i < len(nums); i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		left, right := i+1, len(nums)-1
		for left < right {
			temp := nums[i] + nums[left] + nums[right]
			if temp > 0 {
				right--
			} else if temp < 0 {
				left++
			} else {
				result = append(result, []int{nums[i], nums[left], nums[right]})
				for left < right && nums[left] == nums[left+1] {
					left++
				}
				for left < right && nums[right] == nums[right-1] {
					right--
				}
				left++
				right--
			}
		}
	}

	return result
}
