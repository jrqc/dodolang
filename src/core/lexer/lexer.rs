use crate::core::token::token::Token;
use crate::core::token::token::TokenType;
use crate::core::token::token::identifier;
use crate::core::lexer::helper::is_letter;
use crate::core::lexer::helper::is_digit;

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
        while self.current_char == ' ' {
            self.read_char()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current_char {

            // OPERATOR
            '=' => Token::new(TokenType::ASSIGN, self.current_char.to_string()),
            '+' => Token::new(TokenType::PLUS, self.current_char.to_string()),
            '-' => Token::new(TokenType::MINUS, self.current_char.to_string()),
            '*' => Token::new(TokenType::ASTERISK, self.current_char.to_string()),
            '/' => Token::new(TokenType::SLASH, self.current_char.to_string()),

            // DELIMITER
            '{' => Token::new(TokenType::LeftBrace, self.current_char.to_string()),
            '}' => Token::new(TokenType::RightBrace, self.current_char.to_string()),
            '[' => Token::new(TokenType::LeftBracket, self.current_char.to_string()),
            ']' => Token::new(TokenType::RightBracket, self.current_char.to_string()),
            '(' => Token::new(TokenType::LeftParenthesis, self.current_char.to_string()),
            ')' => Token::new(TokenType::RightParenthesis, self.current_char.to_string()),
            ':' => Token::new(TokenType::COLON, self.current_char.to_string()),
            ',' => Token::new(TokenType::COMMA, self.current_char.to_string()),

            // NEW LINE
            '\n' => Token::new(TokenType::NewLine, self.current_char.to_string()),

            // COMMENT
            '#' => Token::new(TokenType::COMMENT, self.current_char.to_string()),

            // EOF
            '\0' => Token::new(TokenType::EOF, self.current_char.to_string()),

            _ => {
                if is_letter(self.current_char) {
                    let val = self.read_identifier();
                    return Token::new(identifier(val), val.to_string());
                } else if is_digit(self.current_char) {
                    return Token::new(TokenType::INT, self.read_number().to_string());
                } else {
                    Token::new(TokenType::ILLEGAL, self.current_char.to_string())
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
    use crate::core::token::token::Token;
    use crate::core::token::token::TokenType;

    #[test]
    fn basic_delimiters() {
        let input = "=+(){}[],:\n";

        let expected = [
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::PLUS, "+".to_string()),
            Token::new(TokenType::LeftParenthesis, "(".to_string()),
            Token::new(TokenType::RightParenthesis, ")".to_string()),
            Token::new(TokenType::LeftBrace, "{".to_string()),
            Token::new(TokenType::RightBrace, "}".to_string()),
            Token::new(TokenType::LeftBracket, "[".to_string()),
            Token::new(TokenType::RightBracket, "]".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::COLON, ":".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::EOF, "\0".to_string()),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for token in expected.iter() {
            let lexed_token = lexer.next_token();

            assert_eq!(lexed_token, *token);
        }

    }


    #[test]
    fn basic_assignment() {
        let input =
            "scalar x
            vector y[2]
             matrix z[2,2]
             z = { 1 1 0 0 }
             print x";

        let expected = [
            Token::new(TokenType::SCALAR, "scalar".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::VECTOR, "vector".to_string()),
            Token::new(TokenType::IDENT, "y".to_string()),
            Token::new(TokenType::LeftBracket, "[".to_string()),
            Token::new(TokenType::INT, "2".to_string()),
            Token::new(TokenType::RightBracket, "]".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::MATRIX, "matrix".to_string()),
            Token::new(TokenType::IDENT, "z".to_string()),
            Token::new(TokenType::LeftBracket, "[".to_string()),
            Token::new(TokenType::INT, "2".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::INT, "2".to_string()),
            Token::new(TokenType::RightBracket, "]".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::IDENT, "z".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::LeftBrace, "{".to_string()),
            Token::new(TokenType::INT, "1".to_string()),
            Token::new(TokenType::INT, "1".to_string()),
            Token::new(TokenType::INT, "0".to_string()),
            Token::new(TokenType::INT, "0".to_string()),
            Token::new(TokenType::RightBrace, "}".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::PRINT, "print".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::EOF, "\0".to_string()),

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

