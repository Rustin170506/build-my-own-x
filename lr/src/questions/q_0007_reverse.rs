pub fn reverse(x: i32) -> i32 {
    let mut result = 0;
    let mut x = x;
    while x != 0 {
        let digit = x % 10;
        x /= 10;

        if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit >= i32::MAX % 10) {
            return 0;
        }
        if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit <= i32::MIN % 10) {
            return 0;
        }

        result = (result * 10) + digit;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(321), 123);
        assert_eq!(reverse(-321), -123);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(1534236469), 0);
    }
}
