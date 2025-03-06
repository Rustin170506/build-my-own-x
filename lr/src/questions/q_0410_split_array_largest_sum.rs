pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let (mut left, mut right) = (
        nums.iter().cloned().max().unwrap(),
        nums.iter().cloned().sum(),
    );
    while left < right {
        let mid = left + (right - left) / 2;
        let mut count = 1;
        let mut sum = 0;

        for num in &nums {
            if sum + num > mid {
                count += 1;
                sum = *num;
            } else {
                sum += num;
            }
        }

        if count > k {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

#[test]
fn test_split_array() {
    let nums = vec![7, 2, 5, 10, 8];
    let k = 2;
    let result = split_array(nums, k);
    assert_eq!(result, 18);

    let nums = vec![1, 4, 4];
    let k = 3;
    let result = split_array(nums, k);
    assert_eq!(result, 4);

    let nums = vec![1, 2, 3, 4, 5];
    let k = 1;
    let result = split_array(nums, k);
    assert_eq!(result, 15);
}
