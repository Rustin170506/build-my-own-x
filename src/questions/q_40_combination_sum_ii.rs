pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut result = vec![];

    fn backtrack(
        candidates: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
        pos: usize,
        target: i32,
    ) {
        if target == 0 {
            result.push(cur.clone());
        }

        if target <= 0 {
            return;
        }

        let mut prev = -1;

        for i in pos..candidates.len() {
            if candidates[i] == prev {
                continue;
            }

            cur.push(candidates[i]);
            backtrack(candidates, result, cur, i + 1, target - candidates[i]);
            cur.pop();
            prev = candidates[i];
        }
    }

    backtrack(&candidates, &mut result, &mut vec![], 0, target);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            combination_sum2(vec![1, 2, 3], 3),
            vec![vec![1, 2], vec![3]]
        )
    }
}
