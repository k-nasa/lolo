use super::lexer::Lexer;
use super::token::TokenType::*;
use std::string::*;

const PROMPT: &str = &">>";

pub fn run() {
    loop {
        let input: String = read();
        let mut lexer = Lexer::new(input.clone());

        if input.is_empty() {
            return;
        }

        let mut token = lexer.next_token();

        while token.token_type != EOF {
            println!("{:?}", token);
            token = lexer.next_token();
        }
        println!("{:?}", token);
        println!("{}", PROMPT);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
