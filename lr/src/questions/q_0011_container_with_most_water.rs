pub fn max_area(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut result = 0;
    let (mut left, mut right) = (0, height.len() - 1);
    while left < right {
        let area = (right - left) as i32 * i32::min(height[left], height[right]);
        if area > result {
            result = area;
        }
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![]), 0);
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
