package questions

func rob2(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}

	helper := func(numbers []int) int {
		rob1, rob2 := 0, 0
		for _, n := range numbers {
			temp := max(rob1+n, rob2)
			rob1 = rob2
			rob2 = temp
		}

		return rob2
	}

	return max(helper(nums[1:]), helper(nums[:len(nums)-1]))
}
