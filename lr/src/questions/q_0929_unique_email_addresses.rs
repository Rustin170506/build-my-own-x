pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut set = std::collections::HashSet::new();
    for email in emails {
        let mut parts = email.split('@');
        let local = parts.next().unwrap();
        let domain = parts.next().unwrap();
        let local = local.split('+').next().unwrap();
        let local = local.replace('.', "");
        set.insert(format!("{}@{}", local, domain));
    }
    set.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_unique_emails() {
        assert_eq!(num_unique_emails(vec!["a@leetcode.com".to_string(),]), 1);
        assert_eq!(
            num_unique_emails(vec![
                "a@leetcode.com".to_string(),
                "a1@leetcode.com".to_string(),
            ]),
            2
        );
    }
}
