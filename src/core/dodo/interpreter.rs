use crate::core::ast::stmt::Stmt;
use crate::core::ast::expr::Expr;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;

pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {

        }
    }

    pub fn interpret(&mut self, expr: Expr) -> i32 {
        let value = self.evaluate(expr);
        println!("{}", value);
        return value;
    }

    pub fn evaluate(&mut self, expr: Expr) -> i32 {
        if let Expr::Literal(val) = expr {
            return val;
        }
        if let Expr::Grouping(val) = expr {
            return self.evaluate(*val);
        }
        if let Expr::Binary(left, operator, right) = expr {
            let mut left = self.evaluate(*left);
            let mut right = self.evaluate(*right);
            match operator.token_type {
                TokenType::MINUS => return left - right,
                TokenType::PLUS => return left + right,
                TokenType::ASTERISK => return left * right,
                TokenType::SLASH => return left / right,
                _ => ()
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use crate::core::token::token::Token;
    use crate::core::token::token::TokenType;
    use crate::core::lexer::lexer::Lexer;
    use crate::core::ast::expr::Expr;
    use crate::core::ast::parser::Parser;

    #[test]
    fn basic_delimiters() {
        let input = "(21*5)+3+(6*4)\n";


        // let expected = [];

        let mut tokens = Vec::new();

        let mut lexer = Lexer::new(input.to_string());
        loop {
            let lexed_token = lexer.next_token();
            match lexed_token.token_type {
                TokenType::EOF => break,
                _ => {
                    tokens.push(lexed_token)
                }
            }
        }
        let mut parser = Parser::new(tokens);
        let mut expr = parser.parse();

        let mut interpreter = Interpreter::new();
        let mut val = interpreter.evaluate(expr);

        assert_eq!(val, 132);
    }
}