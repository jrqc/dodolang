use crate::core::token::token::Token;
use crate::core::ast::expr::Expr;
use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Definition(Token, String),
    Expression(Expr),
    Print(Expr),
    FOR(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Vec<Stmt>),
    Comment(String),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}