package questions

func findOrder(numCourses int, prerequisites [][]int) []int {
	preMap := make(map[int][]int, numCourses)
	result := make([]int, 0, numCourses)
	for _, prerequisite := range prerequisites {
		preMap[prerequisite[0]] = append(preMap[prerequisite[0]], prerequisite[1])
	}
	visited := make(map[int]struct{})
	cycle := make(map[int]struct{})
	var dfs func(course int) bool
	dfs = func(course int) bool {
		if _, ok := cycle[course]; ok {
			return false
		}
		if _, ok := visited[course]; ok {
			return true
		}
		cycle[course] = struct{}{}
		for _, p := range preMap[course] {
			if !dfs(p) {
				return false
			}
		}
		delete(cycle, course)
		visited[course] = struct{}{}
		result = append(result, course)
		return true
	}

	for i := 0; i < numCourses; i++ {
		if !dfs(i) {
			return nil
		}
	}

	return result
}
