pub fn check_valid_string(s: String) -> bool {
    let (mut left_min, mut left_max) = (0, 0);

    for c in s.chars() {
        if c == '(' {
            left_min += 1;
            left_max += 1;
        } else if c == ')' {
            left_min -= 1;
            left_max -= 1;
        } else {
            left_min -= 1;
            left_max += 1;
        }

        if left_max < 0 {
            return false;
        }
        if left_min < 0 {
            left_min = 0;
        }
    }

    left_min == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid_string() {
        assert!(check_valid_string("()".to_owned()));
        assert!(check_valid_string("(*)".to_owned()));
        assert!(check_valid_string("(*))".to_owned()));
    }
}
