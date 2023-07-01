pub fn multiply(num1: String, num2: String) -> String {
    let mut result = vec![0; num1.len() + num2.len()];
    let mut num1 = num1.chars().collect::<Vec<char>>();
    let mut num2 = num2.chars().collect::<Vec<char>>();
    num1.reverse();
    num2.reverse();
    for i in 0..num1.len() {
        for j in 0..num2.len() {
            let n1 = num1[i].to_digit(10).unwrap();
            let n2 = num2[j].to_digit(10).unwrap();
            let sum = n1 * n2 + result[i + j];
            result[i + j] = sum % 10;
            result[i + j + 1] += sum / 10;
        }
    }
    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }
    result.reverse();
    result
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_043() {
        assert_eq!(multiply("2".to_string(), "3".to_string()), "6".to_string());
        assert_eq!(
            multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}
