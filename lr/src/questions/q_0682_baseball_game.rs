use std::collections::VecDeque;

pub fn cal_points(operations: Vec<String>) -> i32 {
    // 1 <= operations.len() <= 1000
    assert!(!operations.is_empty() && operations.len() <= 1000);

    let mut points = VecDeque::new();

    for operation in operations {
        match operation.as_str() {
            "+" => {
                debug_assert!(points.len() >= 2);
                let n1 = points.pop_back().unwrap();
                let n2 = points.back().unwrap();
                let new_score = n1 + n2;
                // 3, 1(n2), 2(n1)
                // 3, 1(n2), 2(n1)
                points.push_back(n1);
                points.push_back(new_score);
            }
            "D" => {
                debug_assert!(!points.is_empty());
                let n = points.back().unwrap() * 2;
                points.push_back(n);
            }
            "C" => {
                debug_assert!(!points.is_empty());
                points.pop_back();
            }
            _ => {
                let n = operation.parse::<i32>().expect("parse error");
                points.push_back(n);
            }
        }
    }

    points.iter().sum()
}

#[test]
fn test_cal_points() {
    let operations = vec![
        "5".to_string(),
        "2".to_string(),
        "C".to_string(),
        "D".to_string(),
        "+".to_string(),
    ];
    assert_eq!(cal_points(operations), 30);

    let operations = vec![
        "5".to_string(),
        "-2".to_string(),
        "4".to_string(),
        "C".to_string(),
        "D".to_string(),
        "9".to_string(),
        "+".to_string(),
        "+".to_string(),
    ];
    assert_eq!(cal_points(operations), 27);
}
