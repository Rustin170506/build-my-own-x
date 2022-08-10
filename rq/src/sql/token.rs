use super::{keyword::Keyword, literal::Literal, symbol::Symbol};
use std::{char, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) enum TokenType {
    Literal(Literal),
    Keyword(Keyword),
    Symbol(Symbol),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Literal(literal) => write!(f, "{}", literal),
            TokenType::Keyword(keyword) => write!(f, "{}", keyword),
            TokenType::Symbol(symbol) => write!(f, "{}", symbol),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) struct Token {
    text: String,
    token_type: TokenType,
    end_offset: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_type = match self.token_type {
            TokenType::Literal(_) => "Literal",
            TokenType::Keyword(_) => "Keyword",
            TokenType::Symbol(_) => "Symbol",
        };
        write!(
            f,
            "Token(\"{}\", {}.{}, {})",
            self.text, token_type, self.token_type, self.end_offset
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let token = Token {
            text: "a".to_string(),
            token_type: TokenType::Literal(Literal::STRING),
            end_offset: 1,
        };
        assert_eq!(token.to_string(), "Token(\"a\", Literal.STRING, 1)");
    }
}
