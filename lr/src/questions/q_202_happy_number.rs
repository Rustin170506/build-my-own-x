use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut visit = HashSet::new();

    while !visit.contains(&n) {
        visit.insert(n);
        n = sum_of_squares(n);

        if n == 1 {
            return true;
        }
    }

    false
}

fn sum_of_squares(n: i32) -> i32 {
    let mut n = n;
    let mut result = 0;
    while n != 0 {
        let mut digit = n % 10;
        digit = digit * digit;
        result += digit;
        n /= 10;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(!is_happy(2));
        assert!(is_happy(1));
    }
}
