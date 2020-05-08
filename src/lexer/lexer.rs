use crate::token::token::Token;
use crate::token::token::identifier;
use crate::lexer::is_letter;
use crate::lexer::is_digit;

pub struct Lexer {
    input: String,
    input_chars: Vec<char>,
    position: usize,
    read_position: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.clone(),
            input_chars: input.chars().collect(),
            position: 0,
            read_position: 0,
            current_char: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input_chars[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = '\0'
        } else {
            self.current_char = self.input_chars[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.current_char) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.current_char) {
            self.read_char()
        }
        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn read_string(&mut self) -> &str {
        let start_pos = self.position + 1;
        self.read_char();
        while self.current_char != '"' {
            self.read_char();
        }
        &self.input[start_pos..self.position]
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current_char {

            // OPERATOR
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::ASTERISK,

            // DELIMITER
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            ':' => Token::COLON,
            ',' => Token::COMMA,

            // EOF

            '\0' => Token::EOF,

            _ => {
                if is_letter(self.current_char) {
                    let literal = self.read_identifier();
                    return identifier(literal);
                } else if is_digit(self.current_char) {
                    self.read_number();
                    return Token::INT;
                } else {
                    Token::ILLEGAL
                }
            }
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::token::Token;

    #[test]
    fn basic_delimiters() {
        let input = "=+(){}[],:";

        let expected = [
            Token::ASSIGN,
            Token::PLUS,
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::COMMA,
            Token::COLON,
            Token::EOF
        ];

        let mut lexer = Lexer::new(input.to_string());

        for token in expected.iter() {
            let lexed_token = lexer.next_token();
            println!("Lexed Token: {}", lexed_token);
            println!("Token: {}", token);
            assert_eq!(lexed_token, *token);
        }
    }

    #[test]
    fn complex_delimiters() {
        let input = "=+(){}[][2]";

        let expected = [
            Token::ASSIGN,
            Token::PLUS,
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::LeftBracket,
            Token::INT,
            Token::RightBracket,
            Token::EOF
        ];

        let mut lexer = Lexer::new(input.to_string());

        for token in expected.iter() {
            let lexed_token = lexer.next_token();
            println!("Lexed Token: {}", lexed_token);
            println!("Token: {}", token);
            assert_eq!(lexed_token, *token);
        }
    }

    #[test]
    fn basic_assignment() {
        let input =
            "scalar x
             vector y[2]
             matrix z[2,2]
             z = { 1 1 0 0 } ";

        let expected = [
            Token::SCALAR,
            Token::IDENT,
            Token::VECTOR,
            Token::IDENT,
            Token::LeftBracket,
            Token::INT,
            Token::RightBracket,
            Token::MATRIX,
            Token::IDENT,
            Token::LeftBracket,
            Token::INT,
            Token::COMMA,
            Token::INT,
            Token::RightBracket,
            Token::IDENT,
            Token::ASSIGN,
            Token::LeftBrace,
            Token::INT,
            Token::INT,
            Token::INT,
            Token::INT,
            Token::RightBrace,
            Token::EOF
        ];

        let mut lexer = Lexer::new(input.to_string());

        for token in expected.iter() {
            let lexed_token = lexer.next_token();
            println!("Lexed Token: {}", lexed_token);
            println!("Token: {}", token);
            assert_eq!(lexed_token, *token);
        }
    }

}

