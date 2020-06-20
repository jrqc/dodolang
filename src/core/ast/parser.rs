use crate::core::ast::stmt::Stmt;
use crate::core::ast::expr::Expr;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::dodo::error_types::DodoParseError;
use crate::core::dodo::error_types::throw_error;

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }
    fn expression(&mut self) -> Expr {
        return self.equality();
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        return expr;
    }
    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token in token_types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn check(&mut self, token_type: TokenType) -> bool {
        return match self.is_at_end() {
            true => false,
            _ => self.peek().token_type == token_type
        };
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn is_at_end(&mut self) -> bool {
        match self.peek().token_type {
            TokenType::EOF => true,
            _ => false
        }
    }
    fn peek(&mut self) -> Token {
        let index = self.current.clone();
        return self.tokens[index as usize].clone();
    }

    fn previous(&mut self) -> Token {
        let index = self.current.clone();
        return self.tokens[(index - 1) as usize].clone();
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();
        while self.match_token(vec![TokenType::GREATER, TokenType::LESS]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        return expr;
    }
    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();
        while self.match_token(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        return expr;
    }
    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::ASTERISK, TokenType::SLASH]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }
    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::MINUS, TokenType::BANG]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }
        return self.primary();
    }
    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::INT]) {
            return Expr::Literal(self.previous().val.parse::<i32>().unwrap());
        }
        if self.match_token(vec![TokenType::LeftParenthesis]) {
            let expr = self.expression();
            self.consume(TokenType::RightParenthesis, "Expect )".to_string());
            return Expr::Grouping(Box::new(expr));
        }
        return Expr::Err;
    }
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, DodoParseError> {
        match self.check(token_type) {
            true => return Ok(self.advance()),
            _ => Err(DodoParseError)
        }
    }
    fn error(&mut self, token: Token, message: String) -> DodoParseError {
        throw_error(token, message);
        return DodoParseError;
    }
    fn sync(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::NewLine {
                break;
            }
            match self.peek().token_type {
                TokenType::SCALAR => break,
                TokenType::MATRIX => break,
                TokenType::FOR => break,
                TokenType::PRINT => break,
                _ => ()
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::core::token::token::Token;
    use crate::core::token::token::TokenType;
    use crate::core::lexer::lexer::Lexer;
    use crate::core::ast::expr::Expr;

    #[test]
    fn basic_delimiters() {
        let input = "(6+5)\n";


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
        let mut parser = Parser::new(tokens.clone());
        let mut expr = parser.parse();
        assert_eq!(expr, Expr::Grouping(Box::from(Expr::Binary(Box::new(Expr::Literal(6)), Token { token_type: TokenType::PLUS, val: "+".to_string() }, Box::new(Expr::Literal(5))))));
    }
}