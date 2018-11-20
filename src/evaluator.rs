use crate::object::*;
use crate::parser::ast::*;
use std::io::Result;
use crate::parser::ast::statements::*;
use crate::parser::ast::expressions::*;

pub fn eval(node: impl Node) -> Result<Object> {

    match node.to_ast() {
        AST::Program(x) => eval_statement(x.statements),
        AST::ExpressionStatement(x) => eval(x.expression),
        AST::PrefixExpression(x)  => eval_prefix_expression(x),
        AST::IntegerLiteral(x) => Ok(Object {
            object_type: ObjectType::Integer(x.value),
        }),
        AST::Boolean(x) => Ok(Object {
            object_type: ObjectType::Boolean(x.value)
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

fn eval_prefix_expression(prefix_expression: PrefixExpression) -> Result<Object> {
    let right = eval(*prefix_expression.right);

    match prefix_expression.operator.as_str() {
        "!" => Ok(eval_bang_operator(right?)),
        _ => panic!(),
    }
}

fn eval_bang_operator(right: Object) -> Object {
    match right.object_type {
        ObjectType::Boolean(true) => FALSE,
        ObjectType::Boolean(false) => TRUE,
        ObjectType::Null => TRUE,
        _ => FALSE,
    }
}
