package questions

func twoSum(numbers []int, target int) []int {
	left, right := 0, len(numbers)-1
	for left < right {
		temp := numbers[left] + numbers[right]
		if temp > target {
			right--
		} else if temp < target {
			left++
		} else {
			return []int{left + 1, right + 1}
		}
	}
	panic("unreachable")
}
