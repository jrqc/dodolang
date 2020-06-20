use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: String,
}

impl Token {
    pub fn new(token_type: TokenType, val: String) -> Self {
        Token {
            token_type,
            val,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    NewLine,

    // COMMENT
    COMMENT,

    // IDENTIFIER
    IDENT,
    STRING,
    INT,

    // OPERATOR


    GREATER,
    // >
    LESS,
    // <
    BangEqual,
    // !=
    EqualEqual,
    // ==
    ASSIGN,
    // =
    SLASH,
    // /
    ASTERISK,
    // *
    PLUS,
    // +
    MINUS,
    // -

    //DELIMITERS

    LeftBrace,
    // {
    RightBrace,
    // }
    LeftBracket,
    // [
    RightBracket,
    // ]
    LeftParenthesis,
    // (
    RightParenthesis,
    // )
    COLON,
    // :
    COMMA,
    // ,
    BANG,
    // !

    //KEYWORDS

    SCALAR,
    // Scalar variable declaration
    VECTOR,
    // Vector variable declaration
    MATRIX,
    // Matrix variable declaration
    FOR,
    // For loop statement
    PRINT,
    // Print statement
}

pub fn identifier(identifier: &str) -> TokenType {
    match identifier {
        "scalar" => TokenType::SCALAR,
        "vector" => TokenType::VECTOR,
        "matrix" => TokenType::MATRIX,

        "for" => TokenType::FOR,

        _ => TokenType::IDENT
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::ASTERISK => write!(f, "*"),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token_type {
            TokenType::ASTERISK => write!(f, "*"),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            _ => write!(f, "{:?}", self.token_type),
        }
    }
}