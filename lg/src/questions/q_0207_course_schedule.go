package questions

func canFinish(numCourses int, prerequisites [][]int) bool {
	visited := make(map[int]struct{})
	preMap := make(map[int][]int)
	for _, pre := range prerequisites {
		preMap[pre[1]] = append(preMap[pre[1]], pre[0])
	}

	var dfs func(course int) bool
	dfs = func(course int) bool {
		if _, ok := visited[course]; ok {
			return false
		}
		if pres := preMap[course]; len(pres) == 0 {
			return true
		}
		visited[course] = struct{}{}
		for _, pre := range preMap[course] {
			if !dfs(pre) {
				return false
			}
		}
		delete(visited, course)
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
