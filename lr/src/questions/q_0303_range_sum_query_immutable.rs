struct NumArray {
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sum = Vec::new();
        let mut sum = 0;
        for n in nums {
            sum += n;
            prefix_sum.push(sum);
        }

        Self { prefix_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let right_sum = self.prefix_sum[right as usize];
        let left_sum = if left == 0 {
            0
        } else {
            self.prefix_sum[(left - 1) as usize]
        };

        right_sum - left_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_array() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
}
