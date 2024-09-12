pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut prices = vec![usize::MAX; n as usize];
    prices[src as usize] = 0;

    for _ in 0..k + 1 {
        let mut tmp = prices.clone();

        for flight in &flights {
            let (src, dst, price) = (flight[0] as usize, flight[1] as usize, flight[2] as usize);
            if prices[src] == usize::MAX {
                continue;
            }

            if prices[src] + price < tmp[dst] {
                tmp[dst] = prices[src] + price
            }
        }
        prices = tmp;
    }

    if prices[dst as usize] == usize::MAX {
        -1
    } else {
        prices[dst as usize] as i32
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
