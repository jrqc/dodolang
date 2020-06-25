use crate::core::token::token::Token;
use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(i32),
    Set(Box<Expr>, Token, Box<Expr>),
    Vector(Vec<i128>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Variable(Token),
    Get(String, Token),
    Unary(Token, Box<Expr>),
    Err,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}