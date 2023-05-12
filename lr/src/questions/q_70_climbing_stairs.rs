pub fn climb_stairs(n: i32) -> i32 {
    let (mut one, mut two) = (1, 1);

    for _ in 0..n - 1 {
        let temp = one;
        one += two;
        two = temp
    }

    one
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(5), 8);
    }
}
