use std::collections::HashMap;

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut window = HashMap::new();
    let (mut left, mut result) = (0, 0);

    for right in 0..fruits.len() {
        *window.entry(fruits[right]).or_insert(0) += 1;

        while window.len() > 2 {
            let lf = fruits[left];
            *window.get_mut(&lf).unwrap() -= 1;
            if window[&lf] == 0 {
                window.remove(&lf);
            }
            left += 1;
        }

        result = result.max(right - left + 1);
    }

    result as i32
}

#[test]
fn test_total_fruit() {
    let test_cases = vec![
        (vec![1, 2, 1], 3),
        (vec![0, 1, 2, 2], 3),
        (vec![1, 2, 3, 2, 2], 4),
        (vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4], 5),
    ];
    for (fruits, expected) in test_cases {
        assert_eq!(total_fruit(fruits), expected);
    }
}
