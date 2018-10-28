use super::token::{Token, TokenType::*};

struct Lexer {
    input: String,
    current_position: i32,
    read_position: i32,
    current_ch: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            current_position: 0,
            read_position: 1,
            current_ch: 0 as char,
        };

        lexer.read_char();
        lexer
    }

    fn get_char(&self, position: i32) -> char {
        match self.input.chars().nth(position as usize) {
            Some(x) => x,
            None => 0 as char,
        }
    }

    fn read_char(&mut self) {
        self.current_ch = self.get_char(self.current_position);
        self.current_position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token: Token;

        match self.current_ch {
            '=' => token = Token::new(ASSIGN, self.current_ch.to_string()),
            '+' => token = Token::new(PLUS, self.current_ch.to_string()),
            ';' => token = Token::new(SEMICOLON, self.current_ch.to_string()),
            ',' => token = Token::new(COMMA, self.current_ch.to_string()),
            '(' => token = Token::new(LPAREN, self.current_ch.to_string()),
            ')' => token = Token::new(RPAREN, self.current_ch.to_string()),
            '{' => token = Token::new(LBRACE, self.current_ch.to_string()),
            '}' => token = Token::new(RBRACE, self.current_ch.to_string()),
            eof if eof as u8 == 0 => token = Token::new(EOF, self.current_ch.to_string()),
            _ => token = Token::new(ILLEGAL, "".to_string()),
        };

        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use super::super::token::TokenType::*;
    use super::Lexer;

    #[test]
    fn is_should_analysis_of_arithmetic_symbols() {
        let input = "=+(){},;l".to_string();

        let expects = vec![
            (ASSIGN, "="),
            (PLUS, "+"),
            (LPAREN, "("),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RBRACE, "}"),
            (COMMA, ","),
            (SEMICOLON, ";"),
            (ILLEGAL, ""),
            (EOF, "\u{0}"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }
}
