use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count: HashMap<i32, i32> = HashMap::default();
    let mut frequent = vec![vec![]; nums.len() + 1];
    for num in nums {
        *count.entry(num).or_insert(0) += 1;
    }
    for (num, cnt) in count {
        frequent[cnt as usize].push(num);
    }

    let mut result = vec![];

    for i in (0..frequent.len()).rev() {
        if !frequent[i].is_empty() {
            for n in &frequent[i] {
                result.push(*n);
                if result.len() == k as usize {
                    return result;
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(
            top_k_frequent(vec![3, 3, 1, 1, 2, 2, 2, 3, 3], 2),
            vec![3, 2]
        );
    }
}
