extern crate lolo;

#[cfg(test)]
mod test {
    use lolo::lexer::*;
    use lolo::object::*;
    use lolo::parser::*;

    #[test]
    fn is_should_eval_integer_expression() {
        let test_cases = vec![
            ("5", 5),
            ("-5", -5),
            ("10", 10),
            ("-10", -10),
            ("10 - 10", 0),
            ("10 + 10", 20),
            ("5 * (10 + 10)", 100),
            ("5 * 10 + 10", 60),
        ];

        for t in test_cases {
            let evaluated = test_eval(t.0);
            test_integer_object(&evaluated, t.1);
        }
    }

    #[test]
    fn is_should_eval_boolean_expression() {
        let test_cases = vec![("true", true), ("false", false)];

        for t in test_cases {
            let evaluated = test_eval(t.0);
            test_boolean_object(&evaluated, t.1);
        }
    }

    #[test]
    fn is_should_eval_bang_operator() {
        let test_cases = vec![
            ("!true", false),
            ("!false", true),
            ("true == true", true),
            ("true == false", false),
            ("true != true", false),
            ("true != false", true),
            ("!5", false),
            ("!!5", true),
            ("!!false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 == 1", true),
            ("1 == 2", false),
            ("1 != 1", false),
            ("1 != 2", true),
        ];

        for t in test_cases {
            let evaluated = test_eval(t.0);
            test_boolean_object(&evaluated, t.1);
        }
    }

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        lolo::evaluator::eval(&program).expect("fald eval")
    }

    fn test_integer_object(obj: &Object, expected: i64) {
        assert_eq!(ObjectType::Integer(expected), obj.object_type);
        assert_eq!(expected.to_string(), obj.inspect());
    }

    fn test_boolean_object(obj: &Object, expected: bool) {
        assert_eq!(ObjectType::Boolean(expected), obj.object_type);
        assert_eq!(expected.to_string(), obj.inspect());
    }
}
