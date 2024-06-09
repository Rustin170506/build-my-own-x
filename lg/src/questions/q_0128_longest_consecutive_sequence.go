package questions

func longestConsecutive(nums []int) int {
	numsMap := make(map[int]struct{}, len(nums))
	for _, num := range nums {
		numsMap[num] = struct{}{}
	}
	result := 0
	for _, num := range nums {
		if _, ok := numsMap[num-1]; !ok {
			tempCount := 0
			nextNum := num
			for {
				if _, ok := numsMap[nextNum]; ok {
					tempCount += 1
					nextNum += 1
				} else {
					break
				}
			}
			if tempCount > result {
				result = tempCount
			}
		}
	}
	return result
}
