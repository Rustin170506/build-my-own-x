pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let mut count = 0;
    for i in 0..=(arr.len() as i32 - k) {
        let mut sum = 0;
        for j in 0..k {
            sum += arr[(i + j) as usize];
        }
        if (sum / k) >= threshold {
            count += 1;
        }
    }

    count
}

#[test]
fn test_num_of_subarrays() {
    assert_eq!(num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
}
