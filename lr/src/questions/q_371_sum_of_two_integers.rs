pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = (a & b) << 1;
        a ^= b;
        b = temp;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum() {
        assert_eq!(get_sum(1, 2), 3);
    }
}
