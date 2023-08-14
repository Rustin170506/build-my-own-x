use std::collections::HashSet;

pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let mut good = HashSet::new();

    for t in triplets {
        if t[0] > target[0] || t[1] > target[1] || t[2] > target[2] {
            continue;
        }

        for i in 0..3 {
            if t[i] == target[i] {
                good.insert(i);
            }
        }
    }

    good.len() == 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_triplets() {
        assert!(merge_triplets(
            vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
            vec![2, 7, 5]
        ))
    }
}
