use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[allow(clippy::upper_case_acronyms)]
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

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::LONG => write!(f, "LONG"),
            Literal::DOUBLE => write!(f, "DOUBLE"),
            Literal::STRING => write!(f, "STRING"),
            Literal::IDENTIFIER => write!(f, "IDENTIFIER"),
        }
    }
}
