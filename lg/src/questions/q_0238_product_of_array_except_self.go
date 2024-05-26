package questions

func productExceptSelf(nums []int) []int {
	prefix := make([]int, 0, len(nums))
	for i := 0; i < len(nums); i++ {
		prefix = append(prefix, 1)
	}
	for i := 1; i < len(nums); i++ {
		prefix[i] = prefix[i-1] * nums[i-1]
	}
	postfix := make([]int, 0, len(nums))
	for i := 0; i < len(nums); i++ {
		postfix = append(postfix, 1)
	}
	for i := len(nums) - 2; i >= 0; i-- {
		postfix[i] = postfix[i+1] * nums[i+1]
	}

	result := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		result[i] = prefix[i] * postfix[i]
	}
	return result
}
