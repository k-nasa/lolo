use crate::object::*;
use crate::parser::ast::expressions::*;
use crate::parser::ast::statements::*;
use crate::parser::ast::*;
use std::io::Result;

pub fn eval(node: &impl Node) -> Result<Object> {
    match node.to_ast() {
        AST::Program(x) => eval_statement(x.statements),
        AST::ExpressionStatement(x) => eval(&x.expression),
        AST::PrefixExpression(x) => eval_prefix_expression(&x),
        AST::InfixExpression(x) => eval_infix_expression(&x),
        AST::IntegerLiteral(x) => Ok(Object {
            object_type: ObjectType::Integer(x.value),
        }),
        AST::Boolean(x) => Ok(Object {
            object_type: ObjectType::Boolean(x.value),
        }),
        _ => unimplemented!(),
    }
}

fn eval_statement(stmts: Vec<Statements>) -> Result<Object> {
    let mut result = Ok(Object {
        object_type: ObjectType::Null,
    });

    for stmt in stmts {
        result = match stmt {
            Statements::ExpressionStatement(x) => eval(&x),
            _ => panic!(),
        }
    }

    result
}

fn eval_prefix_expression(prefix_expression: &PrefixExpression) -> Result<Object> {
    let right = eval(&*prefix_expression.right)?;

    match prefix_expression.operator.as_str() {
        "!" => Ok(eval_bang_operator(right)),
        "-" => Ok(eval_minus_prefix(right)),
        _ => panic!(),
    }
}

fn eval_infix_expression(infix_expression: &InfixExpression) -> Result<Object> {
    let right = eval(&*infix_expression.right)?;
    let left = eval(&*infix_expression.left)?;

    if right.is_int() && left.is_int() {
        return Ok(eval_integer_infix_expression(
            &infix_expression.operator,
            right,
            left,
        ));
    }

    match infix_expression.operator.as_str() {
        "==" => Ok(Object::from_bool(
            left.boolean_value() == right.boolean_value(),
        )),
        "!=" => Ok(Object::from_bool(
            left.boolean_value() != right.boolean_value(),
        )),
        _ => Ok(NULL),
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

fn eval_minus_prefix(right: Object) -> Object {
    match right.object_type {
        ObjectType::Integer(i) => Object {
            object_type: ObjectType::Integer(-i),
        },
        ObjectType::Null => NULL,
        _ => panic!("faild: eval minus prefix"),
    }
}

fn eval_integer_infix_expression(operator: &str, right: Object, left: Object) -> Object {
    match operator {
        // integer operator
        "+" => Object::from_int(left.integer_value() + right.integer_value()),
        "-" => Object::from_int(left.integer_value() - right.integer_value()),
        "*" => Object::from_int(left.integer_value() * right.integer_value()),
        "/" => Object::from_int(left.integer_value() / right.integer_value()),

        // boolean operator
        "<" => Object::from_bool(left.integer_value() < right.integer_value()),
        ">" => Object::from_bool(left.integer_value() > right.integer_value()),
        "==" => Object::from_bool(left.integer_value() == right.integer_value()),
        "!=" => Object::from_bool(left.integer_value() != right.integer_value()),
        _ => NULL,
    }
}
