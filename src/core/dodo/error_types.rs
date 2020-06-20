use std::fmt;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;

// Custom error type; can be any type which defined in the current crate
// 💡 In here, we use a simple "unit struct" to simplify the example
pub struct DodoParseError;

// Implement std::fmt::Display for ParseError
impl fmt::Display for DodoParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for ParseError
impl fmt::Debug for DodoParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub fn throw_error(token: Token, message: String) {
    match token.token_type {
        TokenType::EOF => println!("At end"),
        _ => println!("An Error Occurred, Please Try Again!")
    }
}

#[cfg(test)]
mod tests {}