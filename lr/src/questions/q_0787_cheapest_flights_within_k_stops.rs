pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut prices = vec![i32::MAX; n as usize];
    prices[src as usize] = 0;

    for _ in 0..k + 1 {
        let mut tmp = prices.clone();

        for flight in &flights {
            let (s, d, p) = (flight[0], flight[1], flight[2]);
            if prices[s as usize] == i32::MAX {
                continue;
            }
            if prices[s as usize] + p < tmp[d as usize] {
                tmp[d as usize] = prices[s as usize] + p;
            }
        }
        prices = tmp;
    }

    if prices[dst as usize] == i32::MAX {
        -1
    } else {
        prices[dst as usize]
    }
}

#[test]
fn test_find_cheapest_price() {
    let n = 3;
    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let src = 0;
    let dst = 2;
    let k = 1;
    assert_eq!(find_cheapest_price(n, flights, src, dst, k), 200);
}
