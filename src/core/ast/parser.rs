use crate::core::ast::stmt::Stmt;
use crate::core::ast::expr::Expr;
use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::ast::error_types::DodoParseError;


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

    fn expression(&mut self) -> Expr {
        self.assignment()
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

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::COLON, "Expect ;".to_string());
        return Stmt::Expression(expr);
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::COLON, "Expect ;".to_string());
        return Stmt::Print(value);
    }

    fn assignment(&mut self) -> Expr {
        let mut expr = self.addition();
        let equal = vec![TokenType::ASSIGN];

        if self.match_token(equal.clone()) {
            let equals = self.previous();
            let value = self.assignment();
            if let Expr::Variable(token_name) = expr {
                let name = token_name;
                let boxed_value = Box::new(value);
                return Expr::Assign(name, boxed_value);
            } else if let Expr::Get(name, token_name) = expr {
                let get = Expr::Get(name, token_name.clone());
                let boxed_expr = Box::new(get);
                let boxed_value = Box::new(value);
                return Expr::Set(boxed_expr, token_name.clone(), boxed_value);
            }
            return Expr::Err;
        }
        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();
        let minus_plus = vec![TokenType::MINUS, TokenType::PLUS];
        while self.match_token(minus_plus.clone()) {
            let operator = self.previous();
            let right = Box::new(self.multiplication());
            let boxed_expr = Box::new(expr);
            expr = Expr::Binary(boxed_expr, operator, right)
        }
        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.call();
        // todo
        let asterisk = vec![TokenType::ASTERISK];
        while self.match_token(asterisk.clone()) {
            let operator = self.previous();
            let right = Box::new(self.call());
            let boxed_expr = Box::new(expr);
            expr = Expr::Binary(boxed_expr, operator, right);
        }
        expr
    }

    fn function_call(&mut self, callee: Box<Expr>) -> Expr {
        let mut comma_token = vec![TokenType::COMMA];
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenType::RightParenthesis) {
            while self.match_token(comma_token.clone()) {
                if arguments.len() >= 255 {
                    return Expr::Err;
                }
                arguments.push(self.expression())
            }
        }
        let paren = self.consume(TokenType::RightParenthesis, "Expect ) after arguments.".to_string());
        return Expr::FunctionCall(callee, arguments);
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        let left_paren = vec![TokenType::LeftParenthesis];
        loop {
            if self.match_token(left_paren.clone()) {
                let boxed_expr = Box::new(expr);
                expr = self.function_call(boxed_expr)
            } else {
                break;
            }
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        let number = vec![TokenType::INT];
        let identifier = vec![TokenType::IDENT];
        let left_paren = vec![TokenType::LeftParenthesis];

        if self.match_token(number) {
            Expr::Literal(self.previous().val.to_string().parse::<i32>().unwrap());
        }

        if self.match_token(identifier) {
            Expr::Variable(self.previous());
        }
        Expr::Err
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration())
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        let matrix_type = vec![TokenType::MATRIX];
        if self.match_token(matrix_type) {
            self.matrix_declaration();
        }
        self.statement()
    }

    // todo
    fn matrix_declaration(&mut self) -> Stmt {
        let equal_type = vec![TokenType::ASSIGN];
        let name = self.consume(TokenType::MATRIX, "expect variable name".to_string());
        let mut initializer = Expr::Err;
        if self.match_token(equal_type) {
            initializer = self.expression();
        }
        self.consume(TokenType::COLON, "".to_string());
        return Stmt::Definition(name, initializer);
    }

    fn peek(&mut self) -> Token {
        let mut index = self.current.clone();
        return self.tokens[index as usize].clone();
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek().token_type {
            TokenType::EOF => true,
            _ => false
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn previous(&mut self) -> Token {
        let mut index = self.current.clone();
        return self.tokens[(index - 1) as usize].clone();
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.peek().token_type {
            token_type => return true,
            _ => return false
        }
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        match self.check(token_type) {
            true => self.advance(),
            _ => return Token::new(TokenType::ILLEGAL, "".to_string())
        };
        return Token::new(TokenType::ILLEGAL, "".to_string());
    }

    fn parse_error(&mut self, token: Token, message: String) -> Result<(), DodoParseError> {
        Err(DodoParseError)
    }

    //pub fn parse_scalar_definition_statement(&self, token_type: TokenType) -> stmt {}
    //pub fn parse_scalar_definition_stmt(&self, token_type: TokenType) -> stmt {}
}