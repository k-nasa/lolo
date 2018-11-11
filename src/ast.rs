use crate::token::Token;

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

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    ILLEGAL,
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Identifier(ref x) => x.value.to_string(),
            Expression::IntegerLiteral(ref x) => x.token.literal.to_string(),
            Expression::PrefixExpression(ref x) => {
                format!("({}{})", x.operator, x.right.to_string())
            }
            Expression::InfixExpression(ref x) => format!(
                "({} {} {})",
                x.left.to_string(),
                x.operator,
                x.right.to_string()
            ),
            Expression::Boolean(ref x) => x.token.literal.to_string(),
            _ => String::new(),
        }
    }
}

impl Default for Expression {
    fn default() -> Expression {
        Expression::ILLEGAL
    }
}

#[derive(Debug, Clone, Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, Clone, Default)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

#[derive(Debug, Clone, Default)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, Default)]
pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
    pub left: Box<Expression>,
}

#[derive(Debug, Clone, Default)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

#[derive(Debug, Clone, Default)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn to_string(&self) -> String {
        let mut if_string = format!(
            "if {} {}",
            self.condition.to_string(),
            self.consequence.to_string()
        );

        if let Some(alt) = &self.alternative {
            if_string.push_str(&alt.to_string());
        }

        if_string
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statements>,
}

impl BlockStatement {
    pub fn to_string(&self) -> String {
        let mut return_string = String::new();
        for stmt in &self.statements {
            return_string.push_str(&stmt.to_string());
        }

        return_string
    }
}

#[derive(Debug, Clone, Default)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl FunctionLiteral {
    pub fn to_string(&self) -> String {
        let mut return_string = String::new();
        for param in &self.parameters {
            return_string.push_str(&param.value.to_string());
            return_string.push(',');
        }

        format!(
            "{}({}) {}",
            self.token.literal,
            return_string,
            self.body.to_string()
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl CallExpression {
    pub fn to_string(&self) -> String {
        let mut return_string = String::new();
        for arg in &self.arguments {
            return_string.push_str(&arg.to_string());
        }

        format!("{}({})", self.function.to_string(), return_string,)
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for Identifier {
    fn statement_node() {}
}
