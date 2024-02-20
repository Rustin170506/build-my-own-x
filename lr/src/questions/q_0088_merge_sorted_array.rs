#[allow(clippy::ptr_arg)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    assert!(nums1.len() == (m + n) as usize);
    assert!(nums2.len() == n as usize);

    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = m + n - 1;
    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            k -= 1;
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            k -= 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1];
        merge(&mut nums1, 1, &mut vec![], 0);
        assert_eq!(nums1, vec![1]);
        let mut nums1 = vec![0];
        merge(&mut nums1, 0, &mut vec![1], 1);
        assert_eq!(nums1, vec![1]);
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
        let mut nums1 = vec![2, 0];
        merge(&mut nums1, 1, &mut vec![1], 1);
        assert_eq!(nums1, vec![1, 2]);
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        merge(&mut nums1, 3, &mut vec![1, 2, 3], 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
