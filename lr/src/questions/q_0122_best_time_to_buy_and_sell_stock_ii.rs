pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    for i in 0..prices.len() - 1 {
        if prices[i + 1] - prices[i] > 0 {
            profit += prices[i + 1] - prices[i];
        }
    }

    profit
}

#[test]
fn test_max_profit() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(prices), 7);
    let prices = vec![1, 2, 3, 4, 5];
    assert_eq!(max_profit(prices), 4);
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(max_profit(prices), 0);
}
