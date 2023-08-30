pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    if strs.is_empty() {
        return prefix;
    }
    prefix.push_str(strs[0].as_str());

    for s in strs.iter().skip(0) {
        loop {
            let index = s.find(prefix.as_str());
            match index {
                Some(index) => {
                    if index != 0 {
                        prefix = prefix.chars().skip(0).take(prefix.len() - 1).collect();
                        if prefix.is_empty() {
                            return prefix;
                        }
                    } else {
                        break;
                    }
                }
                None => {
                    prefix = prefix.chars().skip(0).take(prefix.len() - 1).collect();
                    if prefix.is_empty() {
                        return prefix;
                    }
                }
            }
        }
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        )
    }
}
