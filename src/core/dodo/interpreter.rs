use crate::core::ast::stmt::Stmt;
use crate::core::ast::expr::Expr;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::ast::stmt::Stmt::Print;
use crate::core::dodo::environment::Environment;

pub struct Interpreter {
    env: Environment
}

impl Interpreter {
    pub fn new(env: Environment) -> Self {
        Interpreter {
            env
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.statement(stmt);
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> i128 {
        if let Expr::Literal(val) = expr {
            return val as i128;
        }
        if let Expr::Grouping(val) = expr.clone() {
            return self.evaluate(*val);
        }
        if let Expr::Binary(left, operator, right) = expr.clone() {
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
        if let Expr::Unary(token, val) = expr.clone() {
            let mut primary = self.evaluate(*val);
            return -primary;
        }
        if let Expr::Assign(token, val) = expr.clone() {
            let mut final_val = self.evaluate(*val);
            self.env.assign(token, vec![final_val as i32]);
        }
        return 0;
    }

    pub fn statement(&mut self, stmt: Stmt) {
        if let Stmt::Expression(expr) = stmt.clone() {
            self.evaluate(expr);
        }
        if let Stmt::Print(expr) = stmt.clone() {
            let printable = self.evaluate(expr);
            println!("{}", printable);
        }
        if let Stmt::Definition(token, val) = stmt.clone() {
            match token.token_type {
                TokenType::SCALAR => self.env.define(val, vec![]),
                TokenType::VECTOR => self.env.define(val, vec![]),
                TokenType::MATRIX => self.env.define(val, vec![]),
                _ => ()
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Interpreter;
//     use crate::core::token::token::Token;
//     use crate::core::token::token::TokenType;
//     use crate::core::lexer::lexer::Lexer;
//     use crate::core::ast::expr::Expr;
//     use crate::core::ast::parser::Parser;
//
//     #[test]
//     fn basic_operations() {
//         let input = "(21*5)+3+(6*4)\n";
//
//
//         // let expected = [];
//
//         let mut tokens = Vec::new();
//
//         let mut lexer = Lexer::new(input.to_string());
//         loop {
//             let lexed_token = lexer.next_token();
//             match lexed_token.token_type {
//                 TokenType::EOF => break,
//                 _ => {
//                     tokens.push(lexed_token)
//                 }
//             }
//         }
//         let mut parser = Parser::new(tokens);
//         let mut expr = parser.parse();
//
//         let mut interpreter = Interpreter::new();
//         let mut val = interpreter.interpret(expr);
//
//         assert_eq!(val, 132);
//     }
//     #[test]
//     fn basic_operations_2() {
//         let input = "(5+5)*4+4\n";
//
//
//         // let expected = [];
//
//         let mut tokens = Vec::new();
//
//         let mut lexer = Lexer::new(input.to_string());
//         loop {
//             let lexed_token = lexer.next_token();
//             match lexed_token.token_type {
//                 TokenType::EOF => break,
//                 _ => {
//                     tokens.push(lexed_token)
//                 }
//             }
//         }
//         let mut parser = Parser::new(tokens);
//         let mut expr = parser.parse();
//
//         let mut interpreter = Interpreter::new();
//         let mut val = interpreter.interpret(expr);
//
//         assert_eq!(val, 44);
//     }
// }