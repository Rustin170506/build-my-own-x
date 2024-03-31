package questions

func longestPalindrome(s string) string {
	res := ""
	resLen := 0
	for i := range s {
		l, r := i, i
		for l >= 0 && r < len(s) && s[l] == s[r] {
			if (r - l + 1) > resLen {
				res = s[l : r+1]
				resLen = r - l + 1
			}
			l -= 1
			r += 1
		}

		l, r = i, i+1
		for l >= 0 && r < len(s) && s[l] == s[r] {
			if (r - l + 1) > resLen {
				res = s[l : r+1]
				resLen = r - l + 1
			}
			l -= 1
			r += 1
		}
	}

	return res
}
