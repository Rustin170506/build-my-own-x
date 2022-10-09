use std::collections::VecDeque;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    assert!(n >= 1);
    let mut stack = VecDeque::new();
    let mut result = Vec::new();
    fn backtrack(
        open_count: usize,
        close_count: usize,
        stack: &mut VecDeque<char>,
        result: &mut Vec<String>,
        n: usize,
    ) {
        if open_count == n && close_count == n {
            result.push(stack.iter().collect());
            return;
        }
        if open_count < n {
            stack.push_back('(');
            backtrack(open_count + 1, close_count, stack, result, n);
            stack.pop_back();
        }
        if open_count - close_count > 0 {
            stack.push_back(')');
            backtrack(open_count, close_count + 1, stack, result, n);
            stack.pop_back();
        }
    }
    backtrack(0, 0, &mut stack, &mut result, n as usize);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(generate_parenthesis(1), vec!["()".to_string()]);
        assert_eq!(
            generate_parenthesis(3),
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ]
        );
    }
}
