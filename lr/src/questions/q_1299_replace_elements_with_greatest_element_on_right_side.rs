pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    let mut right_max = -1;

    for i in (0..arr.len()).rev() {
        let temp = arr[i];
        arr[i] = right_max;
        right_max = i32::max(right_max, temp);
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_elements() {
        assert_eq!(vec![-1], replace_elements(vec![1]));
        assert_eq!(
            vec![18, 6, 6, 6, 1, -1],
            replace_elements(vec![17, 18, 5, 4, 6, 1])
        );
    }
}
