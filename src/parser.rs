use super::ast::*;
use super::lexer::*;
use super::token::*;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

impl Precedence {
    pub fn from_token(token: &TokenType) -> Precedence {
        use self::Precedence::*;
        use super::token::TokenType::*;

        match token {
            EQ => EQUALS,
            NOTEQ => EQUALS,
            GT => LESSGREATER,
            LT => LESSGREATER,
            PLUS => SUM,
            MINUS => SUM,
            SLASH => PRODUCT,
            ASTERISK => PRODUCT,
            _ => LOWEST,
        }
    }
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::ILLEGAL, ""),
            peek_token: Token::new(TokenType::ILLEGAL, ""),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.token_type != TokenType::EOF {
            let stmt = self.parse_statement();
            program.statements.push(stmt);

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Statements {
        match self.current_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Statements {
        let token = self.current_token.clone();

        // FIXME to notify errors messegage
        assert!(self.peek_token_is(&TokenType::IDENT));
        self.next_token();

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        // FIXME to notify errors messegage
        self.expect_peek_token(TokenType::ASSIGN);
        self.next_token();

        let value = self.parse_expression(&Precedence::LOWEST);

        // WIP
        while self.current_token.token_type != TokenType::SEMICOLON {
            self.next_token();
        }

        Statements::LetStatement(LetStatement { token, name, value })
    }

    fn parse_return_statement(&mut self) -> Statements {
        let return_statement = ReturnStatement {
            token: self.current_token.clone(),
            expression: Expression::default(),
        };

        self.next_token();

        while !self.current_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Statements::ReturnStatement(return_statement)
    }

    fn parse_expression_statement(&mut self) -> Statements {
        let stmt = ExpressionStatement {
            token: self.current_token.clone(),
            expression: self.parse_expression(&Precedence::LOWEST),
        };

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Statements::ExpressionStatement(stmt)
    }

    fn parse_expression(&mut self, preceduce: &Precedence) -> Expression {
        let token = self.current_token.clone();
        let mut left = self
            .parse_prefix(token.token_type)
            .expect("failt parse_expression");

        while !self.peek_token_is(&TokenType::SEMICOLON) && self.peek_precedence() > *preceduce {
            let token = self.peek_token.clone();

            self.next_token();
            left = self.parse_infix(token.token_type, left);
        }

        left
    }

    fn parse_prefix(&mut self, token_type: TokenType) -> Option<Expression> {
        use super::token::TokenType::*;

        match token_type {
            IDENT => Some(self.parse_identifier()),
            INT => Some(self.parse_integer_literal()),
            BANG | MINUS => Some(self.parse_prefix_expression()),
            TRUE | FALSE => Some(self.parse_boolean()),
            LPAREN => self.parse_group_expression(),
            IF => self.parse_if_expression(),
            FUNCTION => self.parse_function_literal(),
            _ => None,
        }
    }

    fn parse_infix(&mut self, token_type: TokenType, left: Expression) -> Expression {
        use super::token::TokenType::*;

        match token_type {
            PLUS | MINUS | SLASH | ASTERISK | EQ | NOTEQ | LT | GT => {
                self.parse_infix_expression(left)
            }
            _ => left,
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Expression {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();
        let left = Box::new(left);
        let precedence = self.current_precedence();

        self.next_token();

        let right = Box::new(self.parse_expression(&precedence));

        Expression::InfixExpression(InfixExpression {
            token,
            operator,
            left,
            right,
        })
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        })
    }

    fn parse_integer_literal(&self) -> Expression {
        Expression::IntegerLiteral(IntegerLiteral {
            token: self.current_token.clone(),
            value: self.current_token.literal.parse().unwrap(),
        })
    }

    fn parse_boolean(&self) -> Expression {
        Expression::Boolean(Boolean {
            token: self.current_token.clone(),
            value: self.current_token_is(&TokenType::TRUE),
        })
    }

    fn parse_prefix_expression(&mut self) -> Expression {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        self.next_token();

        let right = Box::new(self.parse_expression(&Precedence::PREFIX));

        Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right,
        })
    }

    fn parse_group_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let exp = self.parse_expression(&Precedence::LOWEST);

        if !self.expect_peek_token(TokenType::RPAREN) {
            // TODO add error message
            return None;
        }

        Some(exp)
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek_token(TokenType::LPAREN) {
            // TODO add error message
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(&Precedence::LOWEST);

        if !self.expect_peek_token(TokenType::RPAREN) {
            // TODO add error message
            return None;
        }

        if !self.expect_peek_token(TokenType::LBRACE) {
            // TODO add error message
            return None;
        }

        let consequence = self.parse_block_statement();
        let mut alternative = None;

        if self.peek_token_is(&TokenType::ELSE) {
            self.next_token();

            if !self.expect_peek_token(TokenType::LBRACE) {
                return None;
            } else {
                alternative = Some(self.parse_block_statement());
            }
        }

        Some(Expression::IfExpression(IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.current_token.clone();
        self.next_token();

        let mut statements = Vec::new();
        while !self.current_token_is(&TokenType::RBRACE) && !self.current_token_is(&TokenType::EOF)
        {
            let stmt = self.parse_statement();
            statements.push(stmt);
            self.next_token();
        }

        BlockStatement { token, statements }
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek_token(TokenType::LPAREN) {
            return None;
        };

        let parameters = match self.parse_function_parameters() {
            Some(x) => x,
            None => return None,
        };

        if !self.expect_peek_token(TokenType::LBRACE) {
            return None;
        };

        let body = self.parse_block_statement();

        Some(Expression::FunctionLiteral(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(&TokenType::RPAREN) {
            self.next_token();
            return Some(identifiers);
        };

        self.next_token();

        identifiers.push(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        });

        while self.peek_token_is(&TokenType::COMMA) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            });
        }

        if !self.expect_peek_token(TokenType::RPAREN) {
            println!("{:?}", self.current_token);
            println!("{:?}", self.peek_token);
            return None;
        }

        Some(identifiers)
    }

    fn current_token_is(&self, t: &TokenType) -> bool {
        self.current_token.token_type == *t
    }

    fn current_precedence(&self) -> Precedence {
        Precedence::from_token(&self.current_token.token_type)
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from_token(&self.peek_token.token_type)
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek_token(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            return true;
        }

        false
    }
}
