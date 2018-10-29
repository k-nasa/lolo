use std::string::ToString;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new<T: ToString>(token_type: TokenType, literal: T) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn from_char(c: char) -> TokenType {
        match c {
            '+' => TokenType::PLUS,
            '-' => TokenType::MINUS,
            '/' => TokenType::SLASH,
            '*' => TokenType::ASTERISK,
            '<' => TokenType::LT,
            '>' => TokenType::GT,
            ',' => TokenType::COMMA,
            ';' => TokenType::SEMICOLON,
            '(' => TokenType::LPAREN,
            ')' => TokenType::RPAREN,
            '{' => TokenType::LBRACE,
            '}' => TokenType::RBRACE,
            '=' => TokenType::ASSIGN,
            '!' => TokenType::BANG,
            '\0' => TokenType::EOF,
            _ => TokenType::ILLEGAL,
        }
    }

    pub fn fron_string(s: &str) -> TokenType {
        match s {
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "/" => TokenType::SLASH,
            "*" => TokenType::ASTERISK,
            "<" => TokenType::LT,
            ">" => TokenType::GT,
            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "=" => TokenType::ASSIGN,
            "!" => TokenType::BANG,
            "==" => TokenType::EQ,
            "!=" => TokenType::NOTEQ,
            "let" => TokenType::LET,
            "fn" => TokenType::FUNCTION,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            "" => TokenType::EOF,
            _ => TokenType::IDENT,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    EQ,
    NOTEQ,

    LT,
    GT,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}
