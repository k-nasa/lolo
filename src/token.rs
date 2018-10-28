#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn fron_string(s: &str) -> TokenType {
        match s {
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "<" => TokenType::LT,
            ">" => TokenType::GT,
            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "let" => TokenType::LET,
            "fn" => TokenType::FUNCTION,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "return" => TokenType::RETURN,
            "" => TokenType::EOF,
            "=" => TokenType::ASSIGN,
            "!" => TokenType::BANG,
            "==" => TokenType::EQ,
            "!=" => TokenType::NOTEQ,
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
    RETURN,
}
