pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut map = std::collections::HashMap::new();
    for n in nums2 {
        while let Some(&top) = stack.last() {
            if top < n {
                map.insert(stack.pop().unwrap(), n);
            } else {
                break;
            }
        }
        stack.push(n);
    }
    nums1
        .into_iter()
        .map(|n| *map.get(&n).unwrap_or(&-1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_element() {
        assert_eq!(
            vec![-1, 3, -1],
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
        assert_eq!(
            vec![3, -1],
            next_greater_element(vec![2, 4], vec![1, 2, 3, 4])
        );
    }
}
