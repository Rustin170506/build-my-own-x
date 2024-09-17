package questions

import "math"

func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	prices := make([]int, n)
	for i := range prices {
		prices[i] = math.MaxInt
	}
	prices[src] = 0

	for i := 0; i < k+1; i++ {
		tempPrices := make([]int, n)
		copy(tempPrices, prices)

		for _, flight := range flights {
			from, to, price := flight[0], flight[1], flight[2]
			if prices[from] == math.MaxInt {
				continue
			}

			if prices[from]+price < tempPrices[to] {
				tempPrices[to] = prices[from] + price
			}
		}
		prices = tempPrices
	}

	if prices[dst] == math.MaxInt {
		return -1
	}
	return prices[dst]
}
