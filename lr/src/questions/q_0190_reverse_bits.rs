pub fn reverse_bits(x: u32) -> u32 {
    let mut res = 0;

    for i in 0..32 {
        let bit = (x >> i) & 1;
        res |= bit << (31 - i)
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(4294967293), 3221225471)
    }
}
