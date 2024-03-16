package questions

import (
	"strings"
	"unicode"
)

func isPalindrome(s string) bool {
	runeSlice := []rune(s)
	i := 0
	j := len(runeSlice) - 1
	for i < j {
		if !isValidChar(runeSlice[i]) {
			i++
			continue
		}
		if !isValidChar(runeSlice[j]) {
			j--
			continue
		}
		if !strings.EqualFold(string(runeSlice[i]), string(runeSlice[j])) {
			return false
		}
		i++
		j--
	}

	return true
}

func isValidChar(charVar rune) bool {
	return unicode.IsLetter(charVar) || unicode.IsDigit(charVar)
}
