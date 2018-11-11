use super::lexer::Lexer;
use super::parser::Parser;
use std::io;
use std::io::Write;
use std::string::*;

pub fn run() {
    loop {
        print!(">> ");
        io::stdout().flush();

        let input: String = read();
        let lexer = Lexer::new(input.clone());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        println!("{}", program.to_string());
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
