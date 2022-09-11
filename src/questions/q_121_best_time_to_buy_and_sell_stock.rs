pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let (mut left, mut right) = (0, 1);
    let mut max = 0;

    while right < prices.len() {
        if prices[left] < prices[right] {
            let profit = prices[right] - prices[left];
            max = i32::max(max, profit);
        } else {
            left = right;
        }
        right += 1;
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
