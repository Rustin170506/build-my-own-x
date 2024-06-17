pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, arr.len() - 1);
    while right - left >= k as usize {
        if i32::abs(arr[left] - x) > i32::abs(arr[right] - x) {
            left += 1;
        } else {
            right -= 1
        }
    }

    Vec::from(&arr[left..=right])
}

#[test]
fn test_find_closest_elements() {
    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, 5),
        vec![2, 3, 4, 5]
    );
}
