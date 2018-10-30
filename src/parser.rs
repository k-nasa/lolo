use super::ast::*;
use super::lexer::*;
use super::token::*;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
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
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            };

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statements> {
        match self.current_token.token_type {
            TokenType::LET => Some(self.parse_let_statement()),
            _ => None,
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

#[cfg(test)]
mod test {
    use super::Lexer;
    use super::Parser;
    use super::Statements;
    use crate::ast::Node;

    #[test]
    fn is_should_parse_let_statement() {
        let input = "
            let x = 5;
            let y = 10;
            let hoge = 89898989;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        let expect_names = ["x", "y", "hoge"];

        for (i, name) in expect_names.iter().enumerate() {
            let smtm = &program.statements[i];

            test_let_statement(smtm, name);
        }
    }

    fn test_let_statement(stmt: &Statements, name: &str) {
        assert_eq!(stmt.token_literal(), "let");

        let let_stmt;
        match stmt {
            Statements::LetStatement(stmt) => let_stmt = stmt,
            _ => panic!(),
        };

        assert_eq!(let_stmt.name.value, name);
        assert_eq!(let_stmt.name.token_literal(), name);
    }
}
