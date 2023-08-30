pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let (mut left, mut right) = (1_i64, *piles.iter().max().unwrap() as i64);
    let mut result = right;

    while left <= right {
        let mid = (left + right) / 2;
        let mut hours = 0_i64;
        piles.iter().for_each(|p| {
            hours += ((*p as i64) + mid - 1) / mid;
        });

        if hours <= h as i64 {
            result = i64::min(result, mid);
            right = mid - 1;
        } else {
            left = mid + 1
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
        assert_eq!(
            min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
    }
}
