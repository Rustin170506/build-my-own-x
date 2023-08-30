use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut res = vec![];

    let digit_to_char = vec![
        ('2', "abc".to_owned()),
        ('3', "def".to_owned()),
        ('4', "ghi".to_owned()),
        ('5', "jkl".to_owned()),
        ('6', "mno".to_owned()),
        ('7', "qprs".to_owned()),
        ('8', "tuv".to_owned()),
        ('9', "wxyz".to_owned()),
    ];
    let digit_to_char: HashMap<char, String> = digit_to_char.into_iter().collect();

    fn dfs(
        digit_to_char: &HashMap<char, String>,
        res: &mut Vec<String>,
        digits: &String,
        i: usize,
        current_str: &mut String,
    ) {
        if current_str.len() == digits.len() {
            res.push(current_str.clone());
            return;
        }

        for c in digit_to_char
            .get(&digits.chars().nth(i).unwrap())
            .unwrap()
            .chars()
        {
            current_str.push(c);
            dfs(digit_to_char, res, digits, i + 1, current_str);
            current_str.pop();
        }
    }

    if !digits.is_empty() {
        dfs(&digit_to_char, &mut res, &digits, 0, &mut "".to_owned());
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            letter_combinations("23".to_owned()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        )
    }
}
