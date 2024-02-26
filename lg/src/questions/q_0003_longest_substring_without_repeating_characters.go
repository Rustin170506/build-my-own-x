package questions

func lengthOfLongestSubstring(s string) int {
	if len(s) == 0 {
		return 0
	}

	set := make(map[rune]struct{}, 2)
	l := 0
	result := 0
	for r, c := range s {
		for _, ok := set[c]; ok; _, ok = set[c] {
			delete(set, c)
			l++
		}
		set[c] = struct{}{}
		result = max(result, r-l+1)
	}

	return result
}
