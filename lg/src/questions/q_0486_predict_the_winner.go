package questions

func predictTheWinner(nums []int) bool {
	var dfs func(i, j int, isMe bool) int
	dfs = func(i, j int, isMe bool) int {
		if i == j {
			if isMe {
				return nums[i]
			} else {
				return 0
			}
		}
		if isMe {
			return max(nums[i]+dfs(i+1, j, false), nums[j]+dfs(i, j-1, false))
		} else {
			return min(dfs(i+1, j, true), dfs(i, j-1, true))
		}
	}
	me := dfs(0, len(nums)-1, true)
	sum := 0
	for _, n := range nums {
		sum += n
	}
	you := sum - me

	return me >= you
}
