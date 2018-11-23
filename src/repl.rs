use super::evaluator::eval;
use super::lexer::Lexer;
use super::parser::Parser;
use std::io::Write;
use std::io::*;
use std::string::*;

pub fn run() -> Result<()> {
    loop {
        print!(">> ");
        stdout().flush()?;

        let input: String = read();

        if input == "exit" {
            println!("God Bey...");
            break Ok(());
        }

        let lexer = Lexer::new(input.clone());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let evalted = eval(program)?;

        println!("{}", evalted.inspect());
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
