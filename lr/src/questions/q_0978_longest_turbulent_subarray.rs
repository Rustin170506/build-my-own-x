pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    let mut result = 1;
    let (mut left, mut right) = (0, 1);
    let mut prev = ' ';
    while right < arr.len() {
        if arr[right - 1] < arr[right] && prev != '<' {
            result = result.max(right - left + 1);
            right += 1;
            prev = '<';
        } else if arr[right - 1] > arr[right] && prev != '>' {
            result = result.max(right - left + 1);
            right += 1;
            prev = '>';
        } else {
            right = if arr[right - 1] == arr[right] {
                right + 1
            } else {
                right
            };
            left = right - 1;
            prev = ' ';
        }
    }

    result as i32
}

#[test]
fn test_max_turbulence_size() {
    assert_eq!(max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
    assert_eq!(max_turbulence_size(vec![4, 8, 12, 16]), 2);
    assert_eq!(max_turbulence_size(vec![100]), 1);
    assert_eq!(max_turbulence_size(vec![9, 9]), 1);
}
