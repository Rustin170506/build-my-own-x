use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut res = 0;
    let chars = s.chars().collect::<Vec<char>>();
    let mut roman_nums = HashMap::new();
    roman_nums.insert('I', 1);
    roman_nums.insert('V', 5);
    roman_nums.insert('X', 10);
    roman_nums.insert('L', 50);
    roman_nums.insert('C', 100);
    roman_nums.insert('D', 500);
    roman_nums.insert('M', 1000);
    let mut pre_value = roman_nums.get(&chars[0]).unwrap();
    for c in chars.iter().skip(1) {
        let value = roman_nums.get(c).unwrap();
        if pre_value < value {
            res -= pre_value;
        } else {
            res += pre_value;
        }
        pre_value = value;
    }
    res += pre_value;
    res
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
