pub fn length_of_last_word(s: String) -> i32 {
    let strs: Vec<&str> = s.as_str().trim().split(' ').collect();
    strs.last().unwrap().len() as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_last_word;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Today is a nice day".to_string()), 3);
        assert_eq!(length_of_last_word("A".to_string()), 1);
        assert_eq!(length_of_last_word("H ".to_string()), 1);
        assert_eq!(length_of_last_word(" A".to_string()), 1);
        assert_eq!(length_of_last_word(" AB ".to_string()), 2);
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}
