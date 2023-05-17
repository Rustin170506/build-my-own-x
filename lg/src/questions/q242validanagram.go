package questions

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	sMap := make(map[rune]uint)

	for _, c := range s {
		if v, ok := sMap[c]; ok {
			sMap[c] = v + 1
		} else {
			sMap[c] = 1
		}
	}

	for _, c := range t {
		if v, ok := sMap[c]; ok {
			if v == 1 {
				delete(sMap, c)
			} else {
				sMap[c] = v - 1
			}
		}
	}

	return len(sMap) == 0
}
