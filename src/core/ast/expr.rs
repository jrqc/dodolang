use crate::core::token::token::Token;

#[derive(Clone)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(i32),
    Set(Box<Expr>, Token, Box<Expr>),
    Vector(Vec<i32>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Variable(Token),
    Get(String, Token),
    Err
}
