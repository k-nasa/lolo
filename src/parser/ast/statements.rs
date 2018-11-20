use super::*;

#[derive(Debug, Clone)]
pub enum Statements {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Statements {
    pub fn to_string(&self) -> String {
        match self {
            Statements::LetStatement(ref x) => x.to_string(),
            Statements::ReturnStatement(ref x) => x.to_string(),
            Statements::ExpressionStatement(ref x) => x.to_string(),
        }
    }
}

impl Node for Statements {
    fn token_literal(&self) -> String {
        match self {
            Statements::LetStatement(ref x) => x.token.literal.clone(),
            Statements::ReturnStatement(ref x) => x.token.literal.clone(),
            Statements::ExpressionStatement(ref x) => x.token.literal.clone(),
        }
    }

    fn to_ast(&self) -> AST {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl LetStatement {
    pub fn to_string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token.literal,
            self.name.value,
            self.value.to_string()
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl ReturnStatement {
    pub fn to_string(&self) -> String {
        format!("{} {};", self.token.literal, self.return_value.to_string(),)
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

