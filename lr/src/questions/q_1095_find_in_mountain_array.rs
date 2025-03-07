pub struct MountainArray {
    data: Vec<i32>,
}
impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        self.data[index as usize]
    }

    fn length(&self) -> i32 {
        self.data.len() as i32
    }
}
#[allow(non_snake_case)]
pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
    let n = mountainArr.length();
    let mut left = 0;
    let mut right = n - 1;

    // Find the peak of the mountain
    while left < right {
        let mid = left + (right - left) / 2;
        if mountainArr.get(mid) < mountainArr.get(mid + 1) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    let peak = left;

    // Search in the increasing part
    left = 0;
    right = peak;
    while left <= right {
        let mid = left + (right - left) / 2;
        match mountainArr.get(mid).cmp(&target) {
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }

    // Search in the decreasing part
    left = peak + 1;
    right = n - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        match mountainArr.get(mid).cmp(&target) {
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Greater => left = mid + 1,
            std::cmp::Ordering::Less => right = mid - 1,
        }
    }

    -1
}

#[test]
fn test_find_in_mountain_array() {
    let mountain_arr = MountainArray {
        data: vec![1, 2, 3, 4, 5, 3, 1],
    };
    assert_eq!(find_in_mountain_array(3, &mountain_arr), 2);
    assert_eq!(find_in_mountain_array(5, &mountain_arr), 4);
    assert_eq!(find_in_mountain_array(0, &mountain_arr), -1);
}
