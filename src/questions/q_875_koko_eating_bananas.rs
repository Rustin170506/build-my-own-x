pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let (mut left, mut right) = (1, *piles.iter().max().unwrap());
    let mut result = right;

    while left <= right {
        let mid = (left + right) / 2;
        let mut hours = 0;
        piles.iter().for_each(|p| {
            hours += (p + mid - 1) / mid;
        });

        if hours <= h {
            result = i32::min(result, mid);
            right = mid - 1;
        } else {
            left = mid + 1
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
