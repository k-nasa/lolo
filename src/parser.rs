use super::ast::*;
use super::lexer::*;
use super::token::*;

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
        assert!(self.peek_token_is(&TokenType::ASSIGN));
        self.next_token();

        // WIP
        while self.current_token.token_type != TokenType::SEMICOLON {
            self.next_token();
        }

        Statements::LetStatement(LetStatement { token, name })
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
        unimplemented!()
    }

    fn current_token_is(&self, t: &TokenType) -> bool {
        self.current_token.token_type == *t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}
