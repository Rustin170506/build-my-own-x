package questions

func search(nums []int, target int) int {
	l, r := 0, len(nums)
	for l < r {
		middle := l + (r-l)/2
		if nums[middle] > target {
			r = middle
		} else if nums[middle] < target {
			l = middle + 1
		} else {
			return middle
		}
	}
	return -1
}
