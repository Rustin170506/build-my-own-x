package questions

import "slices"

func maxProduct(nums []int) int {
	result := slices.Max(nums)
	currentMax, currentMin := 1, 1
	for _, n := range nums {
		if n == 0 {
			currentMax, currentMin = 1, 1
			continue
		}
		temp := currentMax
		currentMax = max(currentMax*n, currentMin*n, n)
		currentMin = min(temp*n, currentMin*n, n)
		result = max(currentMax, result)
	}

	return result
}
