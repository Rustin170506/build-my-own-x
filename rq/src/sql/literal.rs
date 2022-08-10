use std::fmt::Display;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Display, EnumString)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
pub(crate) enum Literal {
    LONG,
    DOUBLE,
    STRING,
    IDENTIFIER,
}

impl Literal {
    fn is_number_start(ch: char) -> bool {
        ch.is_ascii_digit() || ch == '.'
    }

    fn is_identifier_start(ch: char) -> bool {
        ch.is_alphabetic()
    }

    fn is_identifier_part(ch: char) -> bool {
        ch.is_alphanumeric() || ch.is_ascii_digit() || ch == '_'
    }

    fn is_chars_start(ch: char) -> bool {
        ch == '\'' || ch == '"'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(Literal::from_str("LONG"), Ok(Literal::LONG));
        assert_eq!(Literal::from_str("DOUBLE"), Ok(Literal::DOUBLE));
        assert_eq!(Literal::from_str("STRING"), Ok(Literal::STRING));
        assert_eq!(Literal::from_str("IDENTIFIER"), Ok(Literal::IDENTIFIER));
    }

    #[test]
    fn test_is_number_start() {
        assert!(Literal::is_number_start('1'));
        assert!(Literal::is_number_start('.'));
        assert!(Literal::is_number_start('0'));
        assert!(!Literal::is_number_start('a'));
    }

    #[test]
    fn test_is_identifier_start() {
        assert!(Literal::is_identifier_start('a'));
        assert!(Literal::is_identifier_start('A'));
        assert!(!Literal::is_identifier_start('_'));
        assert!(!Literal::is_identifier_start('1'));
    }

    #[test]
    fn test_is_identifier_part() {
        assert!(Literal::is_identifier_part('a'));
        assert!(Literal::is_identifier_part('A'));
        assert!(Literal::is_identifier_part('_'));
        assert!(Literal::is_identifier_part('1'));
    }

    #[test]
    fn test_is_chars_start() {
        assert!(Literal::is_chars_start('\''));
        assert!(Literal::is_chars_start('"'));
        assert!(!Literal::is_chars_start('a'));
    }
}
