use std::collections::VecDeque;

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    if tokens.len() == 1 {
        return tokens[0].parse().unwrap();
    }

    let mut stack = VecDeque::new();
    for token in tokens {
        match token.as_str() {
            "+" => {
                let n2 = stack.pop_back().unwrap();
                let n1 = stack.pop_back().unwrap();
                stack.push_back(n1 + n2);
            }
            "-" => {
                let n2 = stack.pop_back().unwrap();
                let n1 = stack.pop_back().unwrap();
                stack.push_back(n1 - n2);
            }
            "*" => {
                let n2 = stack.pop_back().unwrap();
                let n1 = stack.pop_back().unwrap();
                stack.push_back(n1 * n2);
            }
            "/" => {
                let n2 = stack.pop_back().unwrap();
                let n1 = stack.pop_back().unwrap();
                stack.push_back(n1 / n2);
            }
            operation => {
                let n = operation.parse::<i32>().unwrap();
                stack.push_back(n);
            }
        }
    }

    stack.pop_back().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(eval_rpn(vec!["0".to_string()]), 0);
        assert_eq!(
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
        assert_eq!(
            eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
