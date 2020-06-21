use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::dodo::interpreter;
use crate::core::lexer::lexer::Lexer;
use crate::core::ast::parser::Parser;
use std::io::{self, Write};
use crate::core::dodo::interpreter::Interpreter;

const PROMPT: &str = "$> ";

pub fn start() {
    println!("Dodolang!");

    let mut interpreter = Interpreter::new();

    loop {
        print!("{}", PROMPT);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Err(error) => {
                println!("error: {}", error);
                return;
            }
            Ok(_) => (),
        }

        let mut tokens = Vec::new();

        let mut lexer = Lexer::new(buffer);
        loop {
            let lexed_token = lexer.next_token();
            match lexed_token.token_type {
                TokenType::EOF => break,
                _ => {
                    tokens.push(lexed_token)
                }
            }
        }
        let mut parser = Parser::new(tokens.clone());
        let mut expr = parser.parse();

        let outcome = interpreter.interpret(expr);

        println!("{}", outcome)
    }
}
