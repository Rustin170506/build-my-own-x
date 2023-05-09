pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut par: Vec<usize> = (0..(edges.len() + 1)).collect();
    let mut rank: Vec<i32> = (0..(edges.len() + 1)).map(|_| 1).collect();

    for e in edges {
        if !union(&mut par, &mut rank, e[0], e[1]) {
            return vec![e[0], e[1]];
        }
    }

    unreachable!()
}

fn union(par: &mut [usize], rank: &mut [i32], n1: i32, n2: i32) -> bool {
    let (p1, p2) = (find(par, n1), find(par, n2));
    if p1 == p2 {
        return false;
    }

    if rank[p1] > rank[p2] {
        par[p2] = p1;
        rank[p1] += rank[p2];
    } else {
        par[p1] = p2;
        rank[p2] += rank[p1];
    }

    true
}

fn find(par: &mut [usize], n: i32) -> usize {
    let mut p = par[n as usize];
    while p != par[p] {
        par[p] = par[par[p]];
        p = par[p];
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_redundant_connection() {
        assert_eq!(
            find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
        assert_eq!(
            find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
