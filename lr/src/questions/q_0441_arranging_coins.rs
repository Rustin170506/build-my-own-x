pub fn arrange_coins(n: i32) -> i32 {
    let mut n = n as i64;
    let mut i = 1;
    while n >= i {
        n -= i;
        i += 1;
    }
    (i - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0441() {
        assert_eq!(arrange_coins(5), 2);
        assert_eq!(arrange_coins(8), 3);
    }
}
