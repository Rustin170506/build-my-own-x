use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    assert!(numbers.len() >= 2);
    if numbers.len() == 2 {
        return vec![1, 2];
    }
    let (mut left, mut right) = (0, numbers.len() - 1);
    while left < right {
        match (numbers[left] + numbers[right]).cmp(&target) {
            Ordering::Greater => right -= 1,
            Ordering::Less => left += 1,
            Ordering::Equal => break,
        }
    }

    vec![(left + 1) as i32, (right + 1) as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(two_sum(vec![1, 2], 3), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }
}
