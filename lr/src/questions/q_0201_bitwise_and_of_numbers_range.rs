pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut right = right;
    while left < right {
        right &= right - 1;
    }

    right
}

#[test]
fn test_range_bitwise_and() {
    let left = 5;
    let right = 7;
    let res = range_bitwise_and(left, right);
    assert_eq!(res, 4);
}
