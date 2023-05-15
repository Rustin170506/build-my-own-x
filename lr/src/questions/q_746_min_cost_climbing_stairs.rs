use std::cmp::min;

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut cost = cost;
    cost.push(0);

    for i in cost.len() - 3..0 {
        cost[i] += min(cost[i + 1], cost[i + 2])
    }

    i32::min(cost[0], cost[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 5, 20]), 5);
    }
}
