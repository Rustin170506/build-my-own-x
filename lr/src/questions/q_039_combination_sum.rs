pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut cur = vec![];

    fn dfs(
        index: usize,
        candidates: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
        total: i32,
        target: i32,
    ) {
        if total == target {
            res.push(cur.clone());
            return;
        }
        if index >= candidates.len() || total > target {
            return;
        }

        cur.push(candidates[index]);
        dfs(
            index,
            candidates,
            res,
            cur,
            total + candidates[index],
            target,
        );

        cur.pop();
        dfs(index + 1, candidates, res, cur, total, target);
    }

    dfs(0, &candidates, &mut res, &mut cur, 0, target);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            combination_sum(vec![1, 2, 3], 3),
            vec![vec![1, 1, 1], vec![1, 2], vec![3]]
        )
    }
}
