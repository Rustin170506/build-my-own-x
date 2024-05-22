package questions

func characterReplacement(s string, k int) int {
	count := make(map[rune]int, 0)
	result := 0
	left := 0
	for i, v := range s {
		count[v] += 1
		for i-left+1-getMax(count) > k {
			count[rune(s[left])] -= 1
			left++
		}
		result = max(result, i-left+1)
	}
	return result
}

func getMax(m map[rune]int) int {
	result := 0
	for _, v := range m {
		result = max(result, v)
	}
	return result
}
