use super::token::{Token, TokenType::*};
use super::utils::*;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    current_position: u32,
    read_position: u32,
    current_ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            current_position: 0,
            read_position: 1,
            current_ch: 0 as char,
        };

        lexer.read_char();
        lexer
    }

    fn get_char(&self, position: u32) -> char {
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

    fn read_identifier(&mut self) -> String {
        let start_position = self.current_position as usize - 1;

        while is_letter(self.current_ch) {
            self.read_char();
        }

        let end_position = self.current_position as usize - 1;
        self.input[start_position..end_position].to_string()
    }

    fn read_digit(&mut self) -> String {
        let start = self.current_position as usize - 1;
        while is_digit(self.current_ch) {
            self.read_char();
        }

        let end = self.current_position as usize - 1;
        self.input[start..end].to_string()
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() as u32 {
            '\0'
        } else {
            self.input
                .chars()
                .nth(self.read_position as usize - 1)
                .unwrap()
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_ch == ' ' || self.current_ch == '\n' || self.current_ch == '\t' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;

        self.skip_whitespace();

        match self.current_ch {
            '=' if self.peek_char() == '=' => {
                self.read_char();
                token = Token {
                    literal: "==".to_string(),
                    token_type: EQ,
                };
            }
            '!' if self.peek_char() == '=' => {
                self.read_char();
                token = Token {
                    literal: "!=".to_string(),
                    token_type: NOTEQ,
                };
            }
            ch if is_letter(ch) => {
                let literal = self.read_identifier();
                token = Token {
                    token_type: Token::fron_string(&literal),
                    literal,
                };
                return token;
            }
            ch if is_digit(ch) => {
                return Token {
                    literal: self.read_digit(),
                    token_type: INT,
                }
            }
            ch => token = Token::new(Token::from_char(ch), self.current_ch.to_string()),
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::super::token::TokenType::*;
    use super::Lexer;

    #[test]
    fn is_should_analysis_of_arithmetic_symbols() {
        let input = "=+(){},;".to_string();

        let expects = vec![
            (ASSIGN, "="),
            (PLUS, "+"),
            (LPAREN, "("),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RBRACE, "}"),
            (COMMA, ","),
            (SEMICOLON, ";"),
            (EOF, "\0"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }

    #[test]
    fn is_should_analysis_of_let_fn_literal() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x,y) {
            x + y;
        };
            "
        .to_string();

        let expects = vec![
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INT, "5"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            (EOF, "\0"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }

    #[test]
    fn is_should_analysis_controller_syntax() {
        let input = "
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
            "
        .to_string();

        let expects = vec![
            (IF, "if"),
            (LPAREN, "("),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }

    #[test]
    fn is_should_analysis_of_eq_and_noteq() {
        let input = "
        10 == 10;
        5 != 10;
        "
        .to_string();

        let expects = vec![
            (INT, "10"),
            (EQ, "=="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (INT, "5"),
            (NOTEQ, "!="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (EOF, "\0"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }
}
