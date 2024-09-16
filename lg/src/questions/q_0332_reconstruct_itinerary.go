package questions

import "slices"

func findItinerary(tickets [][]string) []string {
	graph := make(map[string][]string)

	for _, ticket := range tickets {
		graph[ticket[0]] = append(graph[ticket[0]], ticket[1])
	}

	for _, destinations := range graph {
		slices.Sort(destinations)
	}

	route := make([]string, 0)
	var dfs func(airport string)
	dfs = func(airport string) {
		for len(graph[airport]) > 0 {
			next := graph[airport][0]
			graph[airport] = graph[airport][1:]
			dfs(next)
		}
		route = append([]string{airport}, route...)
	}

	dfs("JFK")
	return route
}
