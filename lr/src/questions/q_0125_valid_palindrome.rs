pub fn is_palindrome(s: String) -> bool {
    if !s.is_ascii() {
        return false;
    }

    let s = s.to_lowercase();
    let low_case_s = s.trim();
    let mut temp = String::new();

    for c in low_case_s.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            temp.push(c);
        }
    }

    if temp.is_empty() {
        return true;
    }

    let rev_temp = temp.chars().rev().collect::<String>();
    temp == rev_temp
}

pub fn is_palindrome_two_pointers(s: String) -> bool {
    if !s.is_ascii() {
        return false;
    }

    let s = s.chars().collect::<Vec<_>>();
    let (mut i, mut j) = (0, s.len() - 1);

    while i < j {
        if !s[i].is_alphanumeric() {
            i += 1;
            continue;
        }
        if !s[j].is_alphanumeric() {
            j -= 1;
            continue;
        }
        if s[i].to_lowercase().to_string() != s[j].to_ascii_lowercase().to_string() {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

pub fn is_palindrome_queue(s: String) -> bool {
    use std::collections::VecDeque;
    let mut deque = VecDeque::new();

    for c in s.chars() {
        if c.is_alphanumeric() {
            deque.push_back(c.to_ascii_lowercase());
        }
    }

    while deque.len() > 1 {
        if deque.pop_front() != deque.pop_back() {
            return false;
        }
    }

    true
}

#[test]
fn test_is_palindrome() {
    assert!(!is_palindrome("0P".to_string()));
    assert!(is_palindrome(" ".to_string()));
    assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    assert!(!is_palindrome("race a car".to_string()));
}

#[test]
fn test_is_palindrome_two_pointers() {
    assert!(!is_palindrome_two_pointers("0P".to_string()));
    assert!(is_palindrome_two_pointers(" ".to_string()));
    assert!(is_palindrome_two_pointers(
        "A man, a plan, a canal: Panama".to_string()
    ));
    assert!(!is_palindrome_two_pointers("race a car".to_string()));
}

#[test]
fn test_is_palindrome_quque() {
    assert!(!is_palindrome_queue("0P".to_string()));
    assert!(is_palindrome_queue(" ".to_string()));
    assert!(is_palindrome_queue(
        "A man, a plan, a canal: Panama".to_string()
    ));
    assert!(!is_palindrome_queue("race a car".to_string()));
}
