use crate::core::token::token::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {}


pub struct Variable {
    pub name: String,
    pub size: i32,
}

impl Variable {
    pub fn new(name: String, size: i32) -> Self {
        Variable {
            name,
            size,
        }
    }
}

pub enum Expression {
    Definition(String, i32),
    Integer(i32),
    Infix(TokenType, Box<Expression>, Box<Expression>),
    FunctionCall(),
    Variable(Variable)
}

pub enum Statement {
    Definition(TokenType, Variable),
    Assignment(Variable, Expression),
    Print(Expression),
    Comment(String),
}


#[cfg(test)]
mod tests {
    #[test]
    fn basic_statements() {
        let input = "scalar x
                           vector y[4]";
    }
}