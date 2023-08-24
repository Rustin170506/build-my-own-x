fn encode(list: Vec<String>) -> String {
    let mut res = String::new();
    for s in list {
        res.push_str(&format!("{}{}", s.len(), s));
    }
    res
}

fn decode(s: String) -> Vec<String> {
    let mut res = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let mut j = i;
        while j < s.len() && s[j..].chars().next().unwrap().is_ascii_digit() {
            j += 1;
        }
        let len = s[i..j].parse::<usize>().unwrap();
        res.push(s[j..j + len].to_string());
        i = j + len;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_decode_strings() {
        assert_eq!(decode(encode(vec!["".to_string()])), vec!["".to_string()]);
        assert_eq!(decode(encode(vec!["a".to_string()])), vec!["a".to_string()]);
        assert_eq!(
            decode(encode(vec!["a".to_string(), "b".to_string()])),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(
            decode(encode(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string()
            ])),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert_eq!(
            decode(encode(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ])),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
        assert_eq!(
            decode(encode(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ])),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]
        );
    }
}
