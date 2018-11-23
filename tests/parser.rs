extern crate lolo;

#[cfg(test)]
mod test {
    use lolo::lexer::Lexer;
    use lolo::parser::ast::{expressions::*, statements::*, *};
    use lolo::parser::Parser;

    #[test]
    fn is_should_parse_let_statement() {
        let input = vec![
            ("let x = 5;", "x", "5"),
            ("let y = 10;", "y", "10"),
            ("let hoge = true;", "hoge", "true"),
        ];

        for test in input {
            let lexer = Lexer::new(&test.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            let stmt = &program.statements[0];
            test_let_statement(stmt, test.1);

            let let_stmt = match stmt {
                Statements::LetStatement(x) => x.clone(),
                _ => panic!(),
            };

            assert_eq!(let_stmt.value.to_string(), test.2);
        }
    }

    #[test]
    fn is_should_parse_return_statement() {
        let input = "
            return 5;
            return 10;
            return 89898989;
        ";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        for stmt in program.statements {
            assert_eq!(stmt.token_literal(), "return");
        }
    }

    #[test]
    fn is_should_parse_identifier_expression() {
        let input = "foober;";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.first().unwrap();

        let expression = match stmt {
            Statements::ExpressionStatement(x) => &x.expression,
            _ => panic!(),
        };

        test_identifier(expression, "foober");
    }

    #[test]
    fn is_should_parse_integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.first().unwrap();

        let expression = match stmt {
            Statements::ExpressionStatement(x) => &x.expression,
            _ => panic!(),
        };

        test_integer_literal(&expression, 5)
    }

    #[test]
    fn is_should_parse_prefix_expression() {
        let prefix_tests = vec![("!5;", "!", "5"), ("-15;", "-", "15")];

        for prefix in prefix_tests {
            let lexer = Lexer::new(&prefix.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            let stmt = program.statements.first().unwrap();

            let expression = match stmt {
                Statements::ExpressionStatement(x) => &x.expression,
                _ => panic!(),
            };

            let expression = match expression {
                Expression::PrefixExpression(x) => x,
                _ => panic!(),
            };

            assert_eq!(expression.operator, prefix.1);
            test_integer_literal(&expression.right, prefix.2.parse().unwrap());
        }
    }

    #[test]
    fn is_should_parse_infix_expression() {
        let prefix_tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for prefix in prefix_tests {
            let lexer = Lexer::new(&prefix.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            let stmt = program.statements.first().unwrap();

            let expression = match stmt {
                Statements::ExpressionStatement(x) => &x.expression,
                _ => panic!(),
            };

            let expression = match expression {
                Expression::InfixExpression(x) => x,
                _ => panic!(),
            };

            assert_eq!(expression.operator, prefix.2);
            test_integer_literal(&expression.right, prefix.1);
            test_integer_literal(&expression.left, prefix.3);
        }
    }

    #[test]
    fn is_should_operator_precedence_parsing() {
        let prefix_tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a * b", "((!(-a)) * b)"),
            ("a + b + c", "((a + b) + c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a + b * c", "(a + (b * c))"),
            ("a + b * c - d / e - f", "(((a + (b * c)) - (d / e)) - f)"),
            ("5 > 4 == 4 < 9", "((5 > 4) == (4 < 9))"),
            (
                "3 - 4 * 5 == 2 * 90 + 9",
                "((3 - (4 * 5)) == ((2 * 90) + 9))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 > 5 == true", "((3 > 5) == true)"),
            ("a + (b + c)", "(a + (b + c))"),
            ("(a + b) * c", "((a + b) * c)"),
            ("!(true == false)", "(!(true == false))"),
        ];

        for expected in prefix_tests {
            let lexer = Lexer::new(&expected.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            let actual = program.to_string();
            assert_eq!(actual, expected.1)
        }
    }

    #[test]
    fn is_should_parse_boolean_expression() {
        let tests = vec![
            ("true == true;", true, "==", true),
            ("true != true;", true, "!=", true),
            ("false == false;", false, "==", false),
        ];

        for test in tests {
            let lexer = Lexer::new(&test.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            assert_eq!(program.statements.len(), 1);

            let stmt = program.statements.first().unwrap();

            let expression = match stmt {
                Statements::ExpressionStatement(x) => &x.expression,
                _ => panic!(),
            };

            let expression = match expression {
                Expression::InfixExpression(x) => x,
                _ => panic!(),
            };

            assert_eq!(expression.operator, test.2);
            test_boolean(&expression.right, test.1);
            test_boolean(&expression.left, test.3);
        }
    }

    #[test]
    fn is_should_parse_if_expression() {
        let input = "if (x < y) { x }";
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.first().unwrap();

        let expression = match stmt {
            Statements::ExpressionStatement(x) => &x.expression,
            _ => panic!(),
        };

        let expression = match expression {
            Expression::IfExpression(x) => x,
            _ => panic!(),
        };

        assert_eq!(expression.consequence.statements.len(), 1);
        let consequence = expression.consequence.statements.first().unwrap();

        let expression_statement = match consequence {
            Statements::ExpressionStatement(x) => &x.expression,
            _ => panic!(),
        };

        test_identifier(expression_statement, "x");
        assert_eq!(expression.alternative.is_none(), true);
    }

    #[test]
    fn is_should_parse_function_literal() {
        let input = "fn(x, y) { x + y; }";

        let lexter = Lexer::new(&input);
        let mut parser = Parser::new(lexter);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.first().unwrap();
        let stmt = match stmt {
            Statements::ExpressionStatement(x) => x.expression.clone(),
            _ => panic!(),
        };

        let function = match stmt {
            Expression::FunctionLiteral(x) => x.clone(),
            _ => panic!(),
        };

        assert_eq!(function.parameters.len(), 2);;

        assert_eq!(function.parameters[0].value, "x");
        assert_eq!(function.parameters[1].value, "y");

        assert_eq!(function.body.statements.len(), 1);
    }

    #[test]
    fn is_should_parse_call_expression() {
        let input = "call(1, 2, 3, 4 * 5 * 6)";

        let lexter = Lexer::new(&input);
        let mut parser = Parser::new(lexter);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.first().unwrap();
        let stmt = match stmt {
            Statements::ExpressionStatement(x) => x.expression.clone(),
            _ => panic!(),
        };

        let call = match stmt {
            Expression::CallExpression(x) => x.clone(),
            _ => panic!(),
        };

        assert_eq!(call.arguments.len(), 4);
    }

    fn test_let_statement(stmt: &Statements, name: &str) {
        assert_eq!(stmt.token_literal(), "let");

        let let_stmt;
        match stmt {
            Statements::LetStatement(stmt) => let_stmt = stmt,
            _ => panic!(),
        };

        assert_eq!(let_stmt.name.value, name);
        assert_eq!(let_stmt.name.token.literal, name);
    }

    fn test_integer_literal(expression: &Expression, value: i64) {
        let integer_literal = match expression {
            Expression::IntegerLiteral(x) => x,
            _ => panic!(),
        };

        assert_eq!(integer_literal.value, value);
        assert_eq!(integer_literal.token.literal, value.to_string());
    }

    fn test_identifier(exp: &Expression, value: &str) {
        let ident = match exp {
            Expression::Identifier(ref x) => x.clone(),
            _ => panic!(),
        };

        assert_eq!(ident.value, value);
        assert_eq!(ident.token.literal, value);
    }

    fn test_boolean(exp: &Expression, value: bool) {
        let boolean = match exp {
            Expression::Boolean(ref x) => x.clone(),
            _ => panic!(),
        };

        assert_eq!(boolean.value, value);
        assert_eq!(boolean.token.literal, value.to_string());
    }
}
