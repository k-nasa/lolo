extern crate lolo;

#[cfg(test)]
mod test {
    use lolo::ast::Node;
    use lolo::ast::Statements;
    use lolo::lexer::Lexer;
    use lolo::parser::Parser;

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

    #[test]
    fn is_should_parse_return_statement() {
        let input = "
            return 5;
            return 10;
            return 89898989;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        for stmt in program.statements {
            assert_eq!(stmt.token_literal(), "return");
        }
    }

    #[test]
    fn is_should_parse_identifir_expression() {
        let input = "foober;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];

        let ident = match stmt {
            Statements::ExpressionStatement(x) => &x.expression.identifier,
            _ => panic!(),
        };

        assert_eq!(ident.value, "foober");
        assert_eq!(ident.token_literal(), "foober");
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
