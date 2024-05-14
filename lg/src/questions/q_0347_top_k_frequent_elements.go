package questions

func topKFrequent(nums []int, k int) []int {
	freq := make(map[int]int, len(nums))
	for _, num := range nums {
		freq[num] += 1
	}

	freqArr := make([][]int, len(nums)+1)
	for i := range freqArr {
		freqArr[i] = make([]int, 0)
	}
	for num, count := range freq {
		freqArr[count] = append(freqArr[count], num)
	}

	result := make([]int, 0, len(nums))

	for i := len(freqArr) - 1; i >= 0 && len(result) < k; i-- {
		result = append(result, freqArr[i]...)
	}

	return result
}
