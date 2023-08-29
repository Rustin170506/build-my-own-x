pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut par = (0..n).collect::<Vec<i32>>();
    let mut rank = vec![1; n as usize];
    let mut res = n;
    for e in edges {
        res -= union(e[0], e[1], &mut par, &mut rank);
    }

    res
}

fn union(n1: i32, n2: i32, par: &mut [i32], rank: &mut [i32]) -> i32 {
    let (p1, p2) = (find(n1, par) as usize, find(n2, par) as usize);

    if p1 == p2 {
        return 0;
    }

    if rank[p2] > rank[p1] {
        par[p1] = p2 as i32;
        rank[p2] += rank[p1];
    } else {
        par[p2] = p1 as i32;
        rank[p1] += rank[p2];
    }

    1
}

fn find(n: i32, par: &mut [i32]) -> i32 {
    let mut res = n;
    while res != par[res as usize] {
        par[res as usize] = par[par[res as usize] as usize];
        res = par[res as usize];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_323() {
        assert_eq!(
            2,
            count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]])
        );
        assert_eq!(
            1,
            count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]])
        );
    }
}
