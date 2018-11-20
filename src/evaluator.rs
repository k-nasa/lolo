use crate::object::*;
use crate::parser::ast::*;
use std::io::Result;
use crate::parser::ast::statements::*;

pub fn eval(node: impl Node) -> Result<Object> {

    match node.to_ast() {
        AST::Program(x) => eval_statement(x.statements),
        AST::ExpressionStatement(x) => eval(x.expression),
        AST::IntegerLiteral(x) => Ok(Object {
            object_type: ObjectType::Integer(x.value),
        }),
        _ => {
            panic!()
        }
    }
}

fn eval_statement(stmts: Vec<Statements>) -> Result<Object> {
    let mut result = Ok(Object {
        object_type: ObjectType::Null,
    });

    for stmt in stmts {
        result = match stmt {
            Statements::ExpressionStatement(x) => eval(x),
            _ => panic!(),
        }
    }

    result
}
