package questions

func canFinish(numCourses int, prerequisites [][]int) bool {
	cycle := make(map[int]struct{})
	preMap := make(map[int][]int)
	for _, pre := range prerequisites {
		preMap[pre[0]] = append(preMap[pre[0]], pre[1])
	}

	var dfs func(course int) bool
	dfs = func(course int) bool {
		if _, ok := cycle[course]; ok {
			return false
		}
		if pres := preMap[course]; len(pres) == 0 {
			return true
		}
		cycle[course] = struct{}{}
		for _, pre := range preMap[course] {
			if !dfs(pre) {
				return false
			}
		}
		delete(cycle, course)
		preMap[course] = nil
		return true
	}

	for i := 0; i < numCourses; i++ {
		if !dfs(i) {
			return false
		}
	}

	return true
}
