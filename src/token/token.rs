use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Token {
    ILLEGAL,
    EOF,

    // IDENTIFIER
    IDENT,
    STRING,
    INT,

    // OPERATOR

    ASSIGN,
    // =
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

    //KEYWORDS

    SCALAR,
    // Scalar variable declaration
    VECTOR,
    // Vector variable declaration
    MATRIX,
    // Matrix variable declaration
    FOR,
    // For loop statement
}

pub fn identifier(identifier: &str) -> Token {
    match identifier {
        "scalar" => Token::SCALAR,
        "vector" => Token::VECTOR,
        "matrix" => Token::MATRIX,

        "for" => Token::FOR,

        _ => Token::IDENT
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ASTERISK => write!(f, "*"),
            Token::PLUS => write!(f, "+"),
            Token::MINUS => write!(f, "-"),
            _ => write!(f, "{:?}", self),
        }
    }
}