use crate::core::ast::stmt::Stmt;
use crate::core::ast::expr::Expr;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::dodo::environment::Environment;
use crate::core::dodo::error_types::DodoParseError;
use ndarray::{arr1, arr2, Array1};

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

    pub fn evaluate_literal(&mut self, val: i128) -> i128 {
        return val;
    }

    pub fn evaluate_binary(&mut self, left: Expr, right: Expr, operator: Token) -> Expr {
        println!("{} ---- {}", left, right);
        if let Expr::Literal(val1) = left {
            match right {
                Expr::Literal(val2) => {
                    match operator.token_type {
                        TokenType::MINUS => return Expr::Literal(val1 - val2),
                        TokenType::PLUS => return Expr::Literal(val1 + val2),
                        TokenType::ASTERISK => return Expr::Literal(val1 * val2),
                        TokenType::SLASH => return Expr::Literal(val1 / val2),
                        _ => ()
                    }
                }
                Expr::Vector(token, val2) => {
                    let mut new_vector = vec![0; val2.len()];
                    match operator.token_type {
                        TokenType::ASTERISK => {
                            for (i, item) in val2.iter().enumerate() {
                                new_vector[i] = item * val1
                            }
                            return Expr::Vector(token, new_vector);
                        }
                        TokenType::SLASH => {
                            for (i, item) in val2.iter().enumerate() {
                                new_vector[i] = item / val1
                            }
                            return Expr::Vector(token, new_vector);
                        }
                        _ => ()
                    }
                }
                _ => {}
            }
        } else if let Expr::Literal(val1) = right {
            if let Expr::Literal(val2) = left {
                match operator.token_type {
                    TokenType::MINUS => return Expr::Literal(val1 - val2),
                    TokenType::PLUS => return Expr::Literal(val1 + val2),
                    TokenType::ASTERISK => return Expr::Literal(val1 * val2),
                    TokenType::SLASH => return Expr::Literal(val1 / val2),
                    _ => ()
                }
            }
            if let Expr::Vector(token, val2) = left {
                let mut new_vector = vec![0; val2.len()];
                match operator.token_type {
                    TokenType::ASTERISK => {
                        for (i, item) in val2.iter().enumerate() {
                            new_vector[i] = item * val1
                        }
                        return Expr::Vector(token, new_vector);
                    }
                    TokenType::SLASH => {
                        for (i, item) in val2.iter().enumerate() {
                            new_vector[i] = item / val1
                        }
                        return Expr::Vector(token, new_vector);
                    }
                    _ => ()
                }
            }
        } else if let Expr::Vector(token1, vector1) = right {
            if let Expr::Vector(token2, vector2) = left {
                let mut new_value = 0;
                match operator.token_type {
                    TokenType::ASTERISK => {
                        for (i, item) in vector2.iter().enumerate() {
                            new_value += item * vector1[i]
                        }
                        return Expr::Literal(new_value);
                    }
                    TokenType::SLASH => {
                        for (i, item) in vector2.iter().enumerate() {
                            new_value += item / vector1[i]
                        }
                        return Expr::Literal(new_value);
                    }
                    _ => ()
                }
            }
        }
        else if let Expr::Vector(token1, vector1) = left {
            if let Expr::Vector(token2, vector2) = right {
                let mut new_value = 0;
                match operator.token_type {
                    TokenType::ASTERISK => {
                        for (i, item) in vector2.iter().enumerate() {
                            new_value += item * vector1[i]
                        }
                        return Expr::Literal(new_value);
                    }
                    TokenType::SLASH => {
                        for (i, item) in vector2.iter().enumerate() {
                            new_value += item / vector1[i]
                        }
                        return Expr::Literal(new_value);
                    }
                    _ => ()
                }
            }
        }
        return Expr::Literal(0);
    }

    pub fn evaluate_vector_assignment(&mut self, token: Token, expr: Expr) -> Expr {
        if let Expr::Vector(token1, val) = expr {
            println!("{}", token);
            &self.env.assign(token.clone(), val.clone());
            return Expr::Vector(token.clone(), val.clone());
        }
        return Expr::Vector(token, vec![0]);
    }

    pub fn evaluate_literal_assignment(&mut self, token: Token, expr: Expr) -> Expr {
        println!("wtf");
        if let Expr::Literal(val) = expr {
            &self.env.assign(token, vec![val as i128]);
            return Expr::Literal(val);
        }
        if let Expr::Vector(token1, val) = expr {
            println!("{}", token);
            &self.env.assign(token.clone(), val.clone());
            return Expr::Vector(token.clone(), val.clone());
        }
        return Expr::Literal(0);
    }

    pub fn evaluate_unary(&mut self, expr: Expr) -> Expr {
        if let Expr::Literal(val) = expr {
            return Expr::Literal(-val);
        }
        return Expr::Literal(0);
    }

    pub fn evaluate_variable(&mut self, token: Token) -> Expr {
        let mut final_val = self.env.get(token.clone());
        return if final_val.unwrap().clone().len() > 1 {
            Expr::Vector(token.clone(), final_val.unwrap().clone())
        } else {
            Expr::Literal(final_val.unwrap().clone()[0])
        };
    }

    pub fn evaluate(&mut self, expr: Expr) -> Expr {
        if let Expr::Literal(val) = expr.clone() {
            return expr.clone();
        }
        if let Expr::Vector(token, val) = expr.clone() {
            return expr.clone();
        }
        if let Expr::Matrix(val) = expr.clone() {
            return expr.clone();
        }
        if let Expr::Grouping(val) = expr.clone() {
            return self.evaluate(*val);
        }
        if let Expr::Binary(left, operator, right) = expr.clone() {
            let mut left = self.evaluate(*left);
            let mut right = self.evaluate(*right);
            return self.evaluate_binary(left, right, operator);
        }
        if let Expr::Unary(token, val) = expr.clone() {
            let mut primary = self.evaluate(*val);
            return self.evaluate_unary(primary);
        }
        if let Expr::Assign(token, val, variable_type) = expr.clone() {
            return if let Expr::Vector(token1, vector) = *val.clone() {
                let mut final_expr = self.evaluate(*val);
                println!("asd2");
                self.evaluate_vector_assignment(token, final_expr)
            } else {
                let mut final_expr = self.evaluate(*val);
                self.evaluate_literal_assignment(token, final_expr)
            };
        }
        if let Expr::Variable(token, variable_type) = expr.clone() {
            return self.evaluate_variable(token);
        }
        return Expr::Err;
    }

    pub fn statement(&mut self, stmt: Stmt) {
        if let Stmt::Expression(expr) = stmt.clone() {
            self.evaluate(expr);
        }
        if let Stmt::Print(expr) = stmt.clone() {
            let printable = self.evaluate(expr);
            println!("{}", printable);
        }
        if let Stmt::Definition(token, rows, columns) = stmt.clone() {
            match token.token_type {
                TokenType::SCALAR => self.env.define(token.val, vec![0]),
                TokenType::VECTOR => {
                    self.env.define(token.val, vec![0; rows.parse::<usize>().unwrap()])
                }
                TokenType::MATRIX => {
                    self.env.define(token.val, vec![0; columns.parse::<usize>().unwrap()])
                }
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