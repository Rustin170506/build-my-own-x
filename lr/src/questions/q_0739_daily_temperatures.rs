use std::collections::VecDeque;

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack: VecDeque<(i32, usize)> = VecDeque::new();

    for (i, t) in temperatures.iter().enumerate() {
        while !stack.is_empty() && t > &stack.back().unwrap().0 {
            let (_, index) = stack.pop_back().unwrap();
            result[index] = (i - index) as i32;
        }
        stack.push_back((*t, i));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0])
    }
}
