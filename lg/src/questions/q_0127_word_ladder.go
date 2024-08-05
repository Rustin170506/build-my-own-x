package questions

import "fmt"

func ladderLength(beginWord string, endWord string, wordList []string) int {
	found := false
	for _, s := range wordList {
		if s == endWord {
			found = true
		}
	}

	if !found {
		return 0
	}

	wordList = append(wordList, beginWord)
	neighbor := make(map[string][]string)
	for _, s := range wordList {
		for i := 0; i < len(s); i++ {
			pattern := fmt.Sprintf("%s*%s", s[:i], s[i+1:])
			neighbor[pattern] = append(neighbor[pattern], s)
		}
	}

	visited := make(map[string]struct{})
	queue := make([]string, 0)
	queue = append(queue, beginWord)
	result := 1
	for len(queue) > 0 {
		qLen := len(queue)
		for i := 0; i < qLen; i++ {
			word := queue[0]
			queue = queue[1:]
			if word == endWord {
				return result
			}
			for j := 0; j < len(word); j++ {
				pattern := fmt.Sprintf("%s*%s", word[:j], word[j+1:])
				for _, s := range neighbor[pattern] {
					if _, ok := visited[s]; !ok {
						visited[s] = struct{}{}
						queue = append(queue, s)
					}
				}
			}
		}
		result++
	}
	return 0
}
