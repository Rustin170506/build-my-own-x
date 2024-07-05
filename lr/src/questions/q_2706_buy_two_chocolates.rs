pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let (mut min1, mut min2) = (i32::MAX, i32::MAX);
    for price in prices {
        if price < min1 {
            min2 = min1;
            min1 = price;
        } else if price < min2 {
            min2 = price
        }
    }

    let leftover = money - min1 - min2;
    if leftover >= 0 {
        leftover
    } else {
        money
    }
}

#[test]
fn test_buy_choco() {
    let prices = vec![1, 2, 3, 4];
    let money = 10;
    assert_eq!(buy_choco(prices, money), 7);

    let prices = vec![1, 2, 3, 4];
    let money = 6;
    assert_eq!(buy_choco(prices, money), 3);
}
