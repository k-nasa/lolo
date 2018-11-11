extern crate lolo;

#[cfg(test)]
mod tests {
    use lolo::lexer::token::TokenType::*;
    use lolo::lexer::Lexer;

    #[test]
    fn is_should_analysis_of_arithmetic_symbols() {
        let input = "=+(){},;";

        let expects = vec![
            (ASSIGN, "="),
            (PLUS, "+"),
            (LPAREN, "("),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RBRACE, "}"),
            (COMMA, ","),
            (SEMICOLON, ";"),
            (EOF, "\0"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }

    #[test]
    fn is_should_analysis_of_let_fn_literal() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x,y) {
            x + y;
        };
            ";

        let expects = vec![
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INT, "5"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            (EOF, "\0"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }

    #[test]
    fn is_should_analysis_controller_syntax() {
        let input = "
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
            ";

        let expects = vec![
            (IF, "if"),
            (LPAREN, "("),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }

    #[test]
    fn is_should_analysis_of_eq_and_noteq() {
        let input = "
        10 == 10;
        5 != 10;
        ";

        let expects = vec![
            (INT, "10"),
            (EQ, "=="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (INT, "5"),
            (NOTEQ, "!="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (EOF, "\0"),
        ];

        let mut l = Lexer::new(input);

        for (token_type, literal) in expects {
            let t = l.next_token();

            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal.to_string());
        }
    }
}
