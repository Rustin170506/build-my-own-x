use std::collections::HashMap;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    dfs(0, true, &mut dp, &prices)
}

fn dfs(index: usize, buying: bool, dp: &mut HashMap<(usize, bool), i32>, prices: &[i32]) -> i32 {
    if index >= prices.len() {
        return 0;
    }
    if dp.get(&(index, buying)).is_some() {
        return *dp.get(&(index, buying)).unwrap();
    }
    let cooldown = dfs(index + 1, buying, dp, prices);
    if buying {
        let buy = dfs(index + 1, false, dp, prices) - prices[index];
        let max = buy.max(cooldown);
        dp.insert((index, buying), max);
        max
    } else {
        let sell = dfs(index + 2, true, dp, prices) + prices[index];
        let max = sell.max(cooldown);
        dp.insert((index, buying), max);
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(max_profit(vec![1]), 0);
    }
}
