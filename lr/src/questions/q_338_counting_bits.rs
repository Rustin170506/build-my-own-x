pub fn count_bits(n: i32) -> Vec<i32> {
    let mut result = vec![0; n as usize + 1];
    let mut offset = 1;

    for i in 1..n + 1 {
        if offset * 2 == i {
            offset = i;
        }
        result[i as usize] = 1 + result[(i - offset) as usize];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(2), [0, 1, 1]);
        assert_eq!(count_bits(5), [0, 1, 1, 2, 1, 2]);
    }
}
