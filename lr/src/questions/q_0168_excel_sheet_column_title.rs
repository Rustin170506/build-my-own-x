pub fn convert_to_title(column_number: i32) -> String {
    let mut res = String::new();
    let mut column_number = column_number;

    while column_number > 0 {
        let offset = (column_number - 1) % 26;
        let c = 'A' as u32 + offset as u32;
        res.push(char::from_u32(c).unwrap());
        column_number = (column_number - 1) / 26;
    }

    res.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(convert_to_title(1), "A".to_string());
        assert_eq!(convert_to_title(28), "AB".to_string());
        assert_eq!(convert_to_title(701), "ZY".to_string());
        assert_eq!(convert_to_title(2147483647), "FXSHRXW".to_string());
    }
}
