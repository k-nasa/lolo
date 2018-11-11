pub mod token;

use self::token::{Token, TokenType::*};
use super::utils::*;
use std::string::ToString;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    current_position: u32,
    read_position: u32,
    current_ch: char,
}

impl Lexer {
    pub fn new<T: ToString>(input: T) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
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
        if self.read_position <= self.input.len() as u32 {
            return self
                .input
                .chars()
                .nth(self.read_position as usize - 1)
                .unwrap();
        };

        '\0'
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
            ch => token = Token::new(Token::from_char(ch), self.current_ch),
        };

        self.read_char();
        token
    }
}
