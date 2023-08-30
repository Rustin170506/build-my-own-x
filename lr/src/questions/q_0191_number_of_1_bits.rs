pub fn hamming_weight(n: u32) -> i32 {
    let mut n = n;
    let mut result = 0;

    while n != 0 {
        if n % 2 == 1 {
            result += 1;
        }
        n >>= 1
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(hamming_weight(11), 3)
    }
}
