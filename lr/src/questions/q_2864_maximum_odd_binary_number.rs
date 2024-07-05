pub fn maximum_odd_binary_number(s: String) -> String {
    let one = s.chars().filter(|&a| a == '1').count();
    let zero = s.chars().filter(|&a| a == '0').count();
    let mut result = String::new();

    for _ in 0..one - 1 {
        result.push('1')
    }
    for _ in 0..zero {
        result.push('0')
    }
    result.push('1');

    result
}

#[test]
fn test_maximum_odd_binary_number() {
    let s = "100100".to_string();
    assert_eq!(maximum_odd_binary_number(s), "100001".to_string());

    let s = "1001".to_string();
    assert_eq!(maximum_odd_binary_number(s), "1001".to_string());

    let s = "10".to_string();
    assert_eq!(maximum_odd_binary_number(s), "01".to_string());
}
