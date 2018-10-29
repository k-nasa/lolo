use super::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node();
}

pub trait Expression: Node {
    fn expression_node();
}

// root ast node
pub struct Program {
    pub statements: Vec<Statements>,
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() <= 0 {
            return "".to_string();
        }

        self.statements[0].token_literal()
    }
}

pub enum Statements {
    LetStatement(LetStatement),
}

impl Node for Statements {
    fn token_literal(&self) -> String {
        match self {
            &Statements::LetStatement(ref x) => x.token.literal.clone(),
        }
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node() {}
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for Identifier {
    fn statement_node() {}
}
