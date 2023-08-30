use std::collections::VecDeque;

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = VecDeque::from(nums);
    fn dfs(nums: &mut VecDeque<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![nums.clone().into()];
        }

        let mut res = vec![];
        for i in 0..nums.len() {
            let mut nums = nums.clone();
            let num = nums.remove(i).unwrap();
            let mut sub_res = dfs(&mut nums);
            for sub in sub_res.iter_mut() {
                sub.push(num);
            }
            res.append(&mut sub_res);
        }
        res
    }

    dfs(&mut nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(
            permute(vec![1, 2, 3]),
            vec![
                vec![3, 2, 1],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![1, 2, 3]
            ]
        )
    }
}
