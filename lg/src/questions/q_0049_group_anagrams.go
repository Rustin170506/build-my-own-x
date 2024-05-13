package questions

import "sort"

func groupAnagrams(strs []string) [][]string {
	if len(strs) == 0 {
		return nil
	}
	anagramsMap := make(map[[26]int][]string, len(strs))
	for _, str := range strs {
		key := [26]int{}
		for _, c := range str {
			key[c-'a'] += 1
		}
		if _, ok := anagramsMap[key]; ok {
			anagramsMap[key] = append(anagramsMap[key], str)
		} else {
			anagramsMap[key] = make([]string, 0)
			anagramsMap[key] = append(anagramsMap[key], str)
		}
	}

	result := make([][]string, 0, len(strs))
	for _, v := range anagramsMap {
		result = append(result, v)
	}
	sort.Slice(result, func(i, j int) bool {
		return len(result[i]) > len(result[j])
	})
	return result
}
