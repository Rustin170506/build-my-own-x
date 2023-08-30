use std::collections::VecDeque;

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
    let mut result = 0;
    for (i, &h) in heights.iter().enumerate() {
        let mut start = i;
        while stack.back().is_some() && stack.back().unwrap().1 > h {
            let top: (usize, i32) = stack.pop_back().unwrap();
            let max = top.1 * (i - top.0) as i32;
            if max > result {
                result = max;
            }
            start = top.0;
        }
        stack.push_back((start, h));
    }

    for (i, h) in stack {
        let max = (heights.len() - i) as i32 * h;
        if max > result {
            result = max;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
