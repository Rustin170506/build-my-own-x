use std::{fmt::Display, str::FromStr};

pub(crate) const ALL_SYMBOLS: &[Symbol] = &[
    Symbol::LEFT_PAREN,
    Symbol::RIGHT_PAREN,
    Symbol::LEFT_BRACE,
    Symbol::RIGHT_BRACE,
    Symbol::LEFT_BRACKET,
    Symbol::RIGHT_BRACKET,
    Symbol::SEMI,
    Symbol::COMMA,
    Symbol::DOT,
    Symbol::DOUBLE_DOT,
    Symbol::PLUS,
    Symbol::SUB,
    Symbol::STAR,
    Symbol::SLASH,
    Symbol::QUESTION,
    Symbol::EQ,
    Symbol::GT,
    Symbol::LT,
    Symbol::BANG,
    Symbol::TILDE,
    Symbol::CARET,
    Symbol::PERCENT,
    Symbol::COLON,
    Symbol::DOUBLE_COLON,
    Symbol::COLON_EQ,
    Symbol::LT_EQ,
    Symbol::GT_EQ,
    Symbol::LT_EQ_GT,
    Symbol::LT_GT,
    Symbol::BANG_EQ,
    Symbol::BANG_GT,
    Symbol::BANG_LT,
    Symbol::AMP,
    Symbol::BAR,
    Symbol::DOUBLE_AMP,
    Symbol::DOUBLE_BAR,
    Symbol::DOUBLE_LT,
    Symbol::DOUBLE_GT,
    Symbol::AT,
    Symbol::POUND,
];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) struct Symbol(&'static str);

impl Symbol {
    const LEFT_PAREN: Symbol = Symbol("(");
    const RIGHT_PAREN: Symbol = Symbol(")");
    const LEFT_BRACE: Symbol = Symbol("{");
    const RIGHT_BRACE: Symbol = Symbol("}");
    const LEFT_BRACKET: Symbol = Symbol("[");
    const RIGHT_BRACKET: Symbol = Symbol("]");
    const SEMI: Symbol = Symbol(";");
    const COMMA: Symbol = Symbol(",");
    const DOT: Symbol = Symbol(".");
    const DOUBLE_DOT: Symbol = Symbol("..");
    const PLUS: Symbol = Symbol("+");
    const SUB: Symbol = Symbol("-");
    const STAR: Symbol = Symbol("*");
    const SLASH: Symbol = Symbol("/");
    const QUESTION: Symbol = Symbol("?");
    const EQ: Symbol = Symbol("=");
    const GT: Symbol = Symbol(">");
    const LT: Symbol = Symbol("<");
    const BANG: Symbol = Symbol("!");
    const TILDE: Symbol = Symbol("~");
    const CARET: Symbol = Symbol("^");
    const PERCENT: Symbol = Symbol("%");
    const COLON: Symbol = Symbol(":");
    const DOUBLE_COLON: Symbol = Symbol("::");
    const COLON_EQ: Symbol = Symbol(":=");
    const LT_EQ: Symbol = Symbol("<=");
    const GT_EQ: Symbol = Symbol(">=");
    const LT_EQ_GT: Symbol = Symbol("<==>");
    const LT_GT: Symbol = Symbol("<>");
    const BANG_EQ: Symbol = Symbol("!=");
    const BANG_GT: Symbol = Symbol("!>");
    const BANG_LT: Symbol = Symbol("!<");
    const AMP: Symbol = Symbol("&");
    const BAR: Symbol = Symbol("|");
    const DOUBLE_AMP: Symbol = Symbol("&&");
    const DOUBLE_BAR: Symbol = Symbol("||");
    const DOUBLE_LT: Symbol = Symbol("<<");
    const DOUBLE_GT: Symbol = Symbol(">>");
    const AT: Symbol = Symbol("@");
    const POUND: Symbol = Symbol("#");

    fn is_symbol(ch: char) -> bool {
        ALL_SYMBOLS.iter().any(|symbol| symbol.0.contains(ch))
    }

    fn is_symbol_start(ch: char) -> bool {
        Symbol::is_symbol(ch)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) struct ParseError(String);

impl FromStr for Symbol {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        if let Some(symbol) = ALL_SYMBOLS.iter().find(|&symbol| symbol.0 == s) {
            Ok(*symbol)
        } else {
            Err(ParseError(format!("{} is not a valid symbol", s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Symbol::from_str("("), Ok(Symbol::LEFT_PAREN));
        assert_eq!(Symbol::from_str(")"), Ok(Symbol::RIGHT_PAREN));
        assert_eq!(Symbol::from_str("{"), Ok(Symbol::LEFT_BRACE));
        assert_eq!(Symbol::from_str("}"), Ok(Symbol::RIGHT_BRACE));
    }

    #[test]
    fn test_is_symbol_start() {
        assert!(Symbol::is_symbol_start('('));
        assert!(Symbol::is_symbol_start(')'));
        assert!(Symbol::is_symbol_start('{'));
        assert!(Symbol::is_symbol_start('}'));
        assert!(Symbol::is_symbol_start('['));
        assert!(Symbol::is_symbol_start(']'));
        assert!(Symbol::is_symbol_start(';'));
        assert!(Symbol::is_symbol_start(','));
        assert!(Symbol::is_symbol_start('.'));
        assert!(Symbol::is_symbol_start('+'));
        assert!(Symbol::is_symbol_start('-'));
        assert!(Symbol::is_symbol_start('*'));
        assert!(Symbol::is_symbol_start('/'));
        assert!(Symbol::is_symbol_start('?'));
        assert!(Symbol::is_symbol_start('='));
        assert!(Symbol::is_symbol_start('>'));
        assert!(Symbol::is_symbol_start('<'));
        assert!(Symbol::is_symbol_start('!'));
        assert!(Symbol::is_symbol_start('~'));
        assert!(Symbol::is_symbol_start('^'));
        assert!(Symbol::is_symbol_start('%'));
        assert!(Symbol::is_symbol_start(':'));
        assert!(Symbol::is_symbol_start(':'));
        assert!(Symbol::is_symbol_start('='));
        assert!(Symbol::is_symbol_start('<'));
        assert!(Symbol::is_symbol_start('>'));
        assert!(Symbol::is_symbol_start('<'));
        assert!(Symbol::is_symbol_start('>'));
        assert!(!Symbol::is_symbol_start('a'));
    }
}
