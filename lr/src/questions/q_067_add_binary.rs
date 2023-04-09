pub fn add_binary(a: String, b: String) -> String {
    match (a.as_str(), b.as_str()) {
        ("", "") => "".to_string(),
        (_, "") => a,
        ("", _) => b,
        _ => {
            let (longer_one, shorter_one) = if a.len() > b.len() {
                (
                    a.chars().collect::<Vec<char>>(),
                    b.chars().collect::<Vec<char>>(),
                )
            } else {
                (
                    b.chars().collect::<Vec<char>>(),
                    a.chars().collect::<Vec<char>>(),
                )
            };
            let mut result = String::new();
            let mut carry = '0';
            for i in (0..shorter_one.len()).rev() {
                match (
                    longer_one[i + (longer_one.len() - shorter_one.len())],
                    shorter_one[i],
                    carry,
                ) {
                    ('1', '0', '0') => {
                        carry = '0';
                        result.push('1')
                    }
                    ('0', '1', '0') => {
                        carry = '0';
                        result.push('1')
                    }
                    ('1', '0', '1') => {
                        carry = '1';
                        result.push('0')
                    }
                    ('0', '1', '1') => {
                        carry = '1';
                        result.push('0')
                    }
                    ('1', '1', '0') => {
                        carry = '1';
                        result.push('0')
                    }
                    ('1', '1', '1') => {
                        carry = '1';
                        result.push('1')
                    }
                    ('0', '0', '0') => {
                        carry = '0';
                        result.push('0')
                    }
                    ('0', '0', '1') => {
                        carry = '0';
                        result.push('1')
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            for i in (0..longer_one.len() - shorter_one.len()).rev() {
                match (longer_one[i], carry) {
                    ('1', '0') => {
                        carry = '0';
                        result.push('1')
                    }
                    ('0', '1') => {
                        carry = '0';
                        result.push('1')
                    }
                    ('1', '1') => {
                        carry = '1';
                        result.push('0')
                    }
                    ('0', '0') => {
                        carry = '0';
                        result.push('0')
                    }
                    _ => unreachable!(),
                }
            }

            if carry == '1' {
                result.push('1')
            }

            result.chars().rev().collect::<String>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::add_binary;

    #[test]
    fn test_add_binary() {
        assert_eq!(add_binary("".to_string(), "".to_string()), "".to_string());
        assert_eq!(add_binary("1".to_string(), "".to_string()), "1".to_string());
        assert_eq!(add_binary("".to_string(), "1".to_string()), "1".to_string());
        assert_eq!(
            add_binary("11".to_string(), "".to_string()),
            "11".to_string()
        );
        assert_eq!(
            add_binary("1".to_string(), "111".to_string()),
            "1000".to_string()
        );
        assert_eq!(
            add_binary("1".to_string(), "0".to_string()),
            "1".to_string()
        );
        assert_eq!(
            add_binary("0".to_string(), "1".to_string()),
            "1".to_string()
        );
        assert_eq!(
            add_binary("1".to_string(), "1".to_string()),
            "10".to_string()
        );
        assert_eq!(
            add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
        assert_eq!(
            add_binary("1000".to_string(), "110010".to_string()),
            "111010".to_string()
        );
        assert_eq!(
            add_binary("100".to_string(), "110010".to_string()),
            "110110".to_string()
        );
    }
}
