mod ast;
mod lexer;
mod parser;
mod repl;
mod token;
mod utils;

fn main() {
    repl::run();
}
