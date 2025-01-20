pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let (len1, len2) = (str1.len(), str2.len());
    fn is_divisor(str1: &String, str2: &String, l: usize) -> bool {
        if str1.len() % l != 0 || str2.len() % l != 0 {
            return false;
        }
        let sub_string = str1[0..l].to_string();
        let f1 = str1.len() / l;
        let f2 = str2.len() / l;
        &sub_string.repeat(f1) == str1 && &sub_string.repeat(f2) == str2
    }
    for i in (1..=usize::min(len1, len2)).rev() {
        if is_divisor(&str1, &str2, i) {
            return str1[0..i].to_string();
        }
    }

    "".to_string()
}

#[test]
fn test_gcd_of_strings() {
    assert_eq!(
        gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
        "ABC".to_string()
    );
    assert_eq!(
        gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
        "AB".to_string()
    );
    assert_eq!(
        gcd_of_strings("LEET".to_string(), "CODE".to_string()),
        "".to_string()
    );
}
