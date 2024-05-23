package questions

import (
	"slices"
)

func minEatingSpeed(piles []int, h int) int {
	left, right := 1, slices.Max(piles)
	result := right
	for left < right {
		mid := left + (right-left)/2
		hours := 0
		for _, pile := range piles {
			if pile%mid > 0 {
				hours += pile/mid + 1
			} else {
				hours += pile / mid
			}
		}
		if hours <= h {
			result = min(result, mid)
			right = mid - 1
		} else {
			left = mid + 1
		}
	}
	return result
}
