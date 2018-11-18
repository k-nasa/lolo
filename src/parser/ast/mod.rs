pub mod statements;
pub mod expressions;

use crate::lexer::token::Token;
use self::statements::*;
use self::expressions::*;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node();
}

pub trait Expr: Node {
    fn expression_node();
}

// root ast node
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statements>,
}

impl Program {
    pub fn to_string(&self) -> String {
        let mut string = String::new();

        for statement in &self.statements {
            string.push_str(&statement.to_string());
        }

        string
    }
}

