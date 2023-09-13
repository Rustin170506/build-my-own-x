use std::collections::VecDeque;

pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut points = VecDeque::new();

    for op in operations {
        match op.as_str() {
            "+" => {
                let n1 = points.pop_back().unwrap();
                let n2 = points.pop_back().unwrap();
                let new_score = n1 + n2;
                points.push_back(n2);
                points.push_back(n1);
                points.push_back(new_score);
            }
            "D" => {
                let n = points.back().unwrap() * 2;
                points.push_back(n);
            }
            "C" => {
                points.pop_back();
            }
            _ => {
                let n = op.parse::<i32>().unwrap();
                points.push_back(n);
            }
        }
    }

    points.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_points() {
        assert_eq!(
            30,
            cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ])
        );
        assert_eq!(
            27,
            cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ])
        );
    }
}
