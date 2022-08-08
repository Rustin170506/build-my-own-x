use std::{char, fmt::Display};
use strum_macros::{Display, EnumString};

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

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub(crate) enum Keyword {
    /**
     * common
     */
    SCHEMA,
    DATABASE,
    TABLE,
    COLUMN,
    VIEW,
    INDEX,
    TRIGGER,
    PROCEDURE,
    TABLESPACE,
    FUNCTION,
    SEQUENCE,
    CURSOR,
    FROM,
    TO,
    OF,
    IF,
    ON,
    FOR,
    WHILE,
    DO,
    NO,
    BY,
    WITH,
    WITHOUT,
    TRUE,
    FALSE,
    TEMPORARY,
    TEMP,
    COMMENT,

    /**
     * create
     */
    CREATE,
    REPLACE,
    BEFORE,
    AFTER,
    INSTEAD,
    EACH,
    ROW,
    STATEMENT,
    EXECUTE,
    BITMAP,
    NOSORT,
    REVERSE,
    COMPILE,

    /**
     * alter
     */
    ALTER,
    ADD,
    MODIFY,
    RENAME,
    ENABLE,
    DISABLE,
    VALIDATE,
    USER,
    IDENTIFIED,

    /**
     * truncate
     */
    TRUNCATE,

    /**
     * drop
     */
    DROP,
    CASCADE,

    /**
     * insert
     */
    INSERT,
    INTO,
    VALUES,

    /**
     * update
     */
    UPDATE,
    SET,

    /**
     * delete
     */
    DELETE,

    /**
     * select
     */
    SELECT,
    DISTINCT,
    AS,
    CASE,
    WHEN,
    ELSE,
    THEN,
    END,
    LEFT,
    RIGHT,
    FULL,
    INNER,
    OUTER,
    CROSS,
    JOIN,
    USE,
    USING,
    NATURAL,
    WHERE,
    ORDER,
    ASC,
    DESC,
    GROUP,
    HAVING,
    UNION,

    /**
     * others
     */
    DECLARE,
    GRANT,
    FETCH,
    REVOKE,
    CLOSE,
    CAST,
    NEW,
    ESCAPE,
    LOCK,
    SOME,
    LEAVE,
    ITERATE,
    REPEAT,
    UNTIL,
    OPEN,
    OUT,
    INOUT,
    OVER,
    ADVISE,
    SIBLINGS,
    LOOP,
    EXPLAIN,
    DEFAULT,
    EXCEPT,
    INTERSECT,
    MINUS,
    PASSWORD,
    LOCAL,
    GLOBAL,
    STORAGE,
    DATA,
    COALESCE,

    /**
     * Types
     */
    CHAR,
    CHARACTER,
    VARYING,
    VARCHAR,
    VARCHAR2,
    INTEGER,
    INT,
    SMALLINT,
    DECIMAL,
    DEC,
    NUMERIC,
    FLOAT,
    REAL,
    DOUBLE,
    PRECISION,
    DATE,
    TIME,
    INTERVAL,
    BOOLEAN,
    BLOB,

    /**
     * Conditionals
     */
    AND,
    OR,
    XOR,
    IS,
    NOT,
    NULL,
    IN,
    BETWEEN,
    LIKE,
    ANY,
    ALL,
    EXISTS,

    /**
     * Functions
     */
    AVG,
    MAX,
    MIN,
    SUM,
    COUNT,
    GREATEST,
    LEAST,
    ROUND,
    TRUNC,
    POSITION,
    EXTRACT,
    LENGTH,
    CHAR_LENGTH,
    SUBSTRING,
    SUBSTR,
    INSTR,
    INITCAP,
    UPPER,
    LOWER,
    TRIM,
    LTRIM,
    RTRIM,
    BOTH,
    LEADING,
    TRAILING,
    TRANSLATE,
    CONVERT,
    LPAD,
    RPAD,
    DECODE,
    NVL,

    /**
     * Constraints
     */
    CONSTRAINT,
    UNIQUE,
    PRIMARY,
    FOREIGN,
    KEY,
    CHECK,
    REFERENCES,
}

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
        todo!()
    }

    fn is_symbol_start(ch: char) -> bool {
        todo!()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
