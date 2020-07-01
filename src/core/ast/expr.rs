use crate::core::token::token::{Token, TokenType};
use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assign(Token, Box<Expr>, String),
    Grouping(Box<Expr>),
    Literal(i128),
    Set(Box<Expr>, Token, Box<Expr>),
    Vector(Token, Vec<i128>),
    Matrix(Vec<Vec<i128>>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Variable(Token, String),
    Get(String, Token),
    Unary(Token, Box<Expr>),
    Err,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}