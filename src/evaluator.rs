use crate::object::*;
use crate::parser::ast::*;
use crate::parser::ast::statements::*;
use crate::parser::ast::expressions::*;

pub fn eval(node: impl Node) -> Object {
    println!("{:?}", node.to_ast());

    match node.to_ast() {
        AST::Program(x) => eval_statement(x.statements),
        AST::ExpressionStatement(x) => eval(x.expression),
        AST::IntegerLiteral(x) => Object {
            object_type: ObjectType::Integer(x.value),
        },
        _ => {
            panic!()
        }
    }
}

fn eval_statement(stmts: Vec<Statements>) -> Object {
    let mut result = Object {
        object_type: ObjectType::Null,
    };

    for stmt in stmts {
        result = match stmt {
            Statements::ExpressionStatement(x) => eval(x),
            _ => panic!(),
        }
    }

    result
}
