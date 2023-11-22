pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let length = nums.len();
    merge_sort(&mut nums, 0, length);
    nums
}

fn merge_sort(nums: &mut [i32], l: usize, r: usize) {
    if r <= l + 1 {
        return;
    }

    let m = l + (r - l) / 2;
    merge_sort(nums, l, m);
    merge_sort(nums, m, r);
    merge(nums, l, m, r);
}

fn merge(nums: &mut [i32], l: usize, m: usize, r: usize) {
    let left = nums[l..m].to_vec();
    let right = nums[m..r].to_vec();

    let (mut i, mut j, mut k) = (l, 0, 0);
    while j < left.len() && k < right.len() {
        if left[j] <= right[k] {
            nums[i] = left[j];
            j += 1;
        } else {
            nums[i] = right[k];
            k += 1;
        }
        i += 1;
    }
    while j < left.len() {
        nums[i] = left[j];
        i += 1;
        j += 1;
    }
    while k < right.len() {
        nums[i] = right[k];
        i += 1;
        k += 1;
    }
}

#[test]
fn test_sort_array() {
    assert_eq!(sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    assert_eq!(sort_array(vec![5, 1, 1, 2, 0, 0]), vec![0, 0, 1, 1, 2, 5]);
}
