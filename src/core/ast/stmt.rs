use crate::core::token::token::Token;
use crate::core::ast::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Definition(Token, Expr),
    Expression(Expr),
    Print(Expr),
    FOR(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Vec<Stmt>),
    Comment(String),
}