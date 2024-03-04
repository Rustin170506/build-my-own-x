package questions

import (
	"container/list"
)

func isValid(s string) bool {

	if len(s)%2 == 1 {
		return false
	}

	pairMap := map[rune]rune{
		')': '(',
		']': '[',
		'}': '{',
	}

	stack := list.New()

	for _, c := range s {
		if _, ok := pairMap[c]; !ok && c != '{' && c != '[' && c != '(' {
			return false
		}

		switch c {
		case '{', '[', '(':
			stack.PushBack(c)
		default:
			if stack.Len() == 0 {
				return false
			}
			if stack.Remove(stack.Back()) != pairMap[c] {
				return false
			}
		}
	}

	return stack.Len() == 0
}
