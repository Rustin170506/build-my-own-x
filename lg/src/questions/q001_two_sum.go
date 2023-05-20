package questions

func twoSum(nums []int, target int) []int {
	res := []int{}

	numsMap := make(map[int][]int)

	for i, n := range nums {
		numsMap[n] = append(numsMap[n], i)
	}

	for i, n := range nums {
		remain := target - n
		if exist, ok := numsMap[remain]; ok {
			for _, v := range exist {
				if i != v {
					res = append(res, i, v)
					return res
				}
			}
		}
	}

	return res
}
