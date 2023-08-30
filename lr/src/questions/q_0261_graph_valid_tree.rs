pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let mut graph = vec![vec![]; n as usize];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    }
    let mut visited = vec![false; n as usize];
    if !dfs(&graph, &mut visited, 0, -1) {
        return false;
    }
    for i in 0..n {
        if !visited[i as usize] {
            return false;
        }
    }
    true
}

fn dfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, node: i32, parent: i32) -> bool {
    if visited[node as usize] {
        return false;
    }
    visited[node as usize] = true;
    for &neighbor in graph[node as usize].iter() {
        if neighbor != parent && !dfs(graph, visited, neighbor, node) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_261() {
        assert_eq!(
            valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]),
            true
        );
        assert_eq!(
            valid_tree(
                5,
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]
            ),
            false
        );
    }
}
