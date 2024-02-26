package questions

func maxProfit(prices []int) int {
	if len(prices) == 1 {
		return 0
	}

	i := 0
	j := 1
	result := 0
	for j < len(prices) {
		currentMaxProfit := prices[j] - prices[i]
		result = max(result, currentMaxProfit)
		if prices[i] >= prices[j] {
			i = j
			j++
		} else {
			j++
		}
	}

	return result
}
