pub mod expressions;
pub mod statements;

use self::expressions::*;
use self::statements::*;
use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Program(Program),
    ExpressionStatement(ExpressionStatement),
    IntegerLiteral(IntegerLiteral),
    Boolean(Boolean),
    Identifier(Identifier),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    BlockStatement(BlockStatement),
    IfExpression(IfExpression),
}

pub trait Node {
    fn token_literal(&self) -> String;
    fn to_ast(&self) -> AST;
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

impl Node for Program {
    fn token_literal(&self) -> String {
        unimplemented!()
    }

    fn to_ast(&self) -> AST {
        AST::Program(self.clone())
    }
}
