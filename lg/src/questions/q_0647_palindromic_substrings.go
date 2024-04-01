package questions

func countSubstrings(s string) int {
	count := 0
	for i := range s {
		// Count odd-length palindromes centered at i
		l, r := i, i
		for l >= 0 && r < len(s) && s[l] == s[r] {
			count++
			l--
			r++
		}

		// Count even-length palindromes centered at i
		l, r = i, i+1
		for l >= 0 && r < len(s) && s[l] == s[r] {
			count++
			l--
			r++
		}
	}

	return count
}
