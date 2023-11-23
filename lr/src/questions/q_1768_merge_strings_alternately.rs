pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();
    let mut length = 0;
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        result.push(c1);
        result.push(c2);
        length += 1;
    }
    if word1.len() > length {
        result.extend(word1.chars().skip(length));
    } else if word2.len() > length {
        result.extend(word2.chars().skip(length));
    }

    result
}

#[test]
fn test_merge_alternately() {
    assert_eq!(
        merge_alternately("abc".to_string(), "pqr".to_string()),
        "apbqcr".to_string()
    );
    assert_eq!(
        merge_alternately("ab".to_string(), "pqrs".to_string()),
        "apbqrs".to_string()
    );
    assert_eq!(
        merge_alternately("abcd".to_string(), "pq".to_string()),
        "apbqcd".to_string()
    );
    assert_eq!(
        merge_alternately("cdf".to_string(), "a".to_string()),
        "cadf".to_string()
    );
}
