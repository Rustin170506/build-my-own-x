pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n = nums1.len() + nums2.len();
    let (mut i, mut j, mut pos, mut prev) = (0, 0, 0, 0);
    let mut cur: i32;
    while i < nums1.len() || j < nums2.len() {
        if i < nums1.len() && (j >= nums2.len() || nums1[i] < nums2[j]) {
            cur = nums1[i];
            i += 1;
        } else {
            cur = nums2[j];
            j += 1;
        }
        if n / 2 == pos {
            if n % 2 == 1 {
                return cur as f64;
            } else {
                return (prev + cur) as f64 / 2.0;
            }
        }
        pos += 1;
        prev = cur;
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert!((find_median_sorted_arrays(vec![1, 3], vec![2]) - 2.0).abs() < f64::EPSILON);
    }
}
