pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return -1;
    }

    if haystack.len() == needle.len() {
        return if haystack == needle { 0 } else { -1 };
    }

    let mut result = -1;
    for (index, c) in haystack.chars().enumerate() {
        if c == needle.chars().next().unwrap() {
            let mut remain_needle = needle.chars().skip(1);
            if index + 1 == haystack.len() && remain_needle.next().is_none() {
                result = index as i32;
                return result;
            }
            for i in index + 1..haystack.len() {
                match remain_needle.next() {
                    Some(n) => {
                        if n != haystack.chars().nth(i).unwrap() {
                            break;
                        }
                    }
                    None => {
                        result = index as i32;
                        return result;
                    }
                }
                if i + 1 == haystack.len() && remain_needle.next().is_none() {
                    result = index as i32;
                    return result;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::str_str;

    #[test]
    fn test_str_str() {
        assert_eq!(str_str("mississippi".to_string(), "pi".to_string()), 9);
        assert_eq!(str_str("aaa".to_string(), "aaa".to_string()), 0);
        assert_eq!(str_str("abc".to_string(), "c".to_string()), 2);
        assert_eq!(str_str("a".to_string(), "a".to_string()), 0);
        assert_eq!(str_str("aaaaa".to_string(), "aaaaab".to_string()), -1);
        assert_eq!(str_str("aaaaa".to_string(), "".to_string()), -1);
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
    }
}
