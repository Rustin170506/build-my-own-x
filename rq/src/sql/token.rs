use super::{keyword::Keyword, literal::Literal, symbol::Symbol};
use std::{char, fmt::Display};

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
