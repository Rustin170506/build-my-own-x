pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut result = 0;
    let (mut left, mut right) = (0, height.len() - 1);
    let (mut max_left, mut max_right) = (height[left], height[right]);

    while left < right {
        if max_left < max_right {
            left += 1;
            max_left = i32::max(max_left, height[left]);
            result += max_left - height[left];
        } else {
            right -= 1;
            max_right = i32::max(max_right, height[right]);
            result += max_right - height[right];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(trap(vec![]), 0);
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
