use crate::core::ast::stmt::Stmt;
use crate::core::ast::expr::Expr;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::dodo::error_types::DodoParseError;
use crate::core::dodo::error_types::throw_error;
use crate::core::token::token::TokenType::VECTOR;

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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.declaration())
        }
        return statements;
    }
    fn declaration(&mut self) -> Stmt {
        if self.match_token(vec![TokenType::SCALAR]) {
            let mut var_type = self.peek().clone();
            return self.scalar_declaration(var_type);
        }
        if self.match_token(vec![TokenType::VECTOR]) {
            let mut token = self.peek().clone();
            return self.vector_declaration(token);
        }
        if self.match_token(vec![TokenType::MATRIX]) {
            let mut token = self.peek().clone();
            return self.matrix_declaration(token);
        }
        return self.statement();
        // todo error
    }

    fn scalar_declaration(&mut self, var_type: Token) -> Stmt {
        let mut name = self.consume(TokenType::IDENT, "expect".to_string());
        let mut token = name.unwrap().clone();
        self.consume(TokenType::NewLine, "Expect".to_string());
        return Stmt::Definition(Token::new(TokenType::SCALAR, var_type.val.to_string()), "0".to_string(), "0".to_string());
    }

    fn vector_declaration(&mut self, var_type: Token) -> Stmt {
        self.consume(TokenType::IDENT, "expect".to_string());
        self.consume(TokenType::LeftBracket, "expect".to_string());
        let size = self.consume(TokenType::INT, "expect".to_string());
        self.consume(TokenType::RightBracket, "expect".to_string());
        self.consume(TokenType::NewLine, "Expect".to_string());
        return Stmt::Definition(Token::new(TokenType::VECTOR, var_type.val.to_string()), size.unwrap().val, "1".to_string());
    }

    fn matrix_declaration(&mut self, var_type: Token) -> Stmt {
        self.consume(TokenType::IDENT, "expect".to_string());
        self.consume(TokenType::LeftBracket, "expect".to_string());
        let rows = self.consume(TokenType::INT, "expect".to_string());
        self.consume(TokenType::COMMA, "expect".to_string());
        let columns = self.consume(TokenType::INT, "expect".to_string());
        self.consume(TokenType::RightBracket, "expect".to_string());
        self.consume(TokenType::NewLine, "Expect".to_string());
        return Stmt::Definition(Token::new(TokenType::MATRIX, var_type.val.to_string()), rows.unwrap().val, columns.unwrap().val);
    }

    fn statement(&mut self) -> Stmt {
        let for_stmt = vec![TokenType::FOR];
        let print_stmt = vec![TokenType::PRINT];
        //todo
        //if self.match_token(for_stmt) {
        //    return self.for_statement;
        //}
        if self.match_token(print_stmt) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::NewLine, "Expect ".to_string());
        return Stmt::Print(value);
    }
    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::NewLine, "Expect ".to_string());
        return Stmt::Expression(expr);
    }
    fn expression(&mut self) -> Expr {
        return self.assignment();
    }
    fn assignment(&mut self) -> Expr {
        let expr = self.addition();
        if self.match_token(vec![TokenType::ASSIGN]) {
            let equals = self.previous();
            let value = self.assignment();
            if let Expr::Variable(token, variable_type) = expr {
                println!("{}", token.val);
                return Expr::Assign(token, Box::new(value), variable_type);
            }
            // todo error(equals, "Invalid");
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
        return match self.at_end() {
            true => false,
            _ => self.peek().token_type == token_type
        };
    }
    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn at_end(&mut self) -> bool {
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
        return self.primary().unwrap();
    }
    fn primary(&mut self) -> Result<Expr, DodoParseError> {
        if self.match_token(vec![TokenType::IDENT]) {
            return Ok(Expr::Variable(self.previous(), "literal".to_string()));
        }
        if self.match_token(vec![TokenType::INT]) {
            return Ok(Expr::Literal(self.previous().val.parse::<i128>().unwrap()));
        }
        if self.match_token(vec![TokenType::LeftParenthesis]) {
            let expr = self.expression();
            self.consume(TokenType::RightParenthesis, "Expect )".to_string());
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        if self.match_token(vec![TokenType::LeftBrace]) {
            if self.peek().token_type == TokenType::LeftBrace {
                let mut vector = Vec::new();
                self.consume(TokenType::INT, "Expect )".to_string());
                vector.push(self.previous().val.parse::<i128>().unwrap());
                loop {
                    if self.match_token(vec![TokenType::COMMA]) {
                        continue;
                    } else if self.match_token(vec![TokenType::INT]) {
                        vector.push(self.previous().val.parse::<i128>().unwrap());
                    } else if self.match_token(vec![TokenType::RightBrace]) {
                        break;
                    } else {
                        return Err(DodoParseError);
                    }
                }
            } else {
                let token = self.previous();
                println!("{}", token);
                let mut vector = Vec::new();
                self.consume(TokenType::INT, "Expect )".to_string());
                vector.push(self.previous().val.parse::<i128>().unwrap());
                loop {
                    if self.match_token(vec![TokenType::COMMA]) {
                        continue;
                    } else if self.match_token(vec![TokenType::INT]) {
                        vector.push(self.previous().val.parse::<i128>().unwrap());
                    } else if self.match_token(vec![TokenType::RightBrace]) {
                        break;
                    } else {
                        return Err(DodoParseError);
                    }
                }
                return Ok(Expr::Vector(token, vector));
            }
        }

        return Err(DodoParseError);
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
        while !self.at_end() {
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
    use crate::core::ast::stmt::Stmt;

    // #[test]
    // fn basic_operations() {
    //     let input = "(6+5)\n";
    //
    //
    //     // let expected = [];
    //
    //     let mut tokens = Vec::new();
    //
    //     let mut lexer = Lexer::new(input.to_string());
    //     loop {
    //         let lexed_token = lexer.next_token();
    //         match lexed_token.token_type {
    //             TokenType::EOF => break,
    //             _ => {
    //                 tokens.push(lexed_token)
    //             }
    //         }
    //     }
    //     let mut parser = Parser::new(tokens.clone());
    //     let mut expr = parser.parse();
    //     assert_eq!(expr, Expr::Grouping(Box::from(Expr::Binary(Box::new(Expr::Literal(6)), Token { token_type: TokenType::PLUS, val: "+".to_string() }, Box::new(Expr::Literal(5))))));
    // }
    #[test]
    fn basic_operations_statement() {
        let input = "print 1\n";


        // let expected = [];

        let mut tokens = Vec::new();

        let mut lexer = Lexer::new(input.to_string());
        loop {
            let lexed_token = lexer.next_token();
            match lexed_token.token_type {
                TokenType::EOF => {
                    tokens.push(lexed_token);
                    break;
                }
                _ => {
                    tokens.push(lexed_token)
                }
            }
        }
        let mut parser = Parser::new(tokens.clone());
        let mut stmts = parser.parse();
        assert_eq!(stmts, vec![Stmt::Print(Expr::Literal(1))]);
    }
}