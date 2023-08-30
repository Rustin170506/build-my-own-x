pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    if digits.is_empty() {
        return vec![];
    }
    let mut result = vec![];
    let mut count = 0;
    let mut is_clean = false;
    for d in digits.iter().rev() {
        count += 1;
        let mut temp = d + 1;
        if temp >= 10 {
            temp %= 10;
            result.push(temp);
        } else {
            is_clean = true;
            result.push(temp);
            break;
        }
    }
    if !is_clean {
        result.push(1);
    }
    for i in (0..digits.len() - count).rev() {
        result.push(digits[i]);
    }
    result.reverse();

    result
}

#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(vec![]), vec![]);
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![9, 8, 9]), vec![9, 9, 0]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
