pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }

    let (mut left, mut right) = (0_i64, ((x / 2) + 1) as i64);

    while left < right {
        let mid: i64 = left + (right - left) / 2 + 1;
        let square: i64 = mid * mid;
        if square == x as i64 {
            return mid as i32;
        } else if square < x as i64 {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left as i32
}

#[cfg(test)]
mod tests {
    use super::my_sqrt;

    #[test]
    fn test_str_str() {
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(2), 1);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
    }
}
