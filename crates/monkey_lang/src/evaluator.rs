use crate::ast::{Expression, Program, Statement};
use crate::object::Object;

/// Evaluates a vector of statements, returning a corresponding vector of objects
/// for each statement
fn eval_program(program: &Program) -> Vec<Object> {
    program
        .statements
        .iter()
        .map(|statement| match statement {
            Statement::Expression(expr) => eval_expression(&expr),
            _ => Object::Null,
        })
        .collect()
}

/// Same as eval_program but converts object to string after
pub fn eval_program_string_output(program: &Program) -> Vec<String> {
    let evaluation = eval_program(program);
    evaluation.iter().map(|obj| obj.to_string()).collect()
}

/// Evaluates an expression
fn eval_expression(expression: &Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(value) => Object::Integer(value.to_owned()),
        Expression::Boolean(bool) => Object::Boolean(bool.to_owned()),
        _ => Object::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::evaluator::{eval_expression, eval_program};
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    fn parse_and_eval_program(input: &str) -> Vec<Object> {
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        eval_program(&program)
    }

    #[test]
    fn eval_integer_expression() {
        let input = "5";
        let evaluation = parse_and_eval_program(input);

        assert_eq!(evaluation.len(), 1);
        assert_eq!(evaluation[0], Object::Integer(5));

        let input = "10";
        let evaluation = parse_and_eval_program(input);

        assert_eq!(evaluation.len(), 1);
        assert_eq!(evaluation[0], Object::Integer(10));
    }

    #[test]
    fn eval_boolean_expression() {
        let input = "true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation.len(), 1);
        assert_eq!(evaluation[0], Object::Boolean(true));

        let input = "false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation.len(), 1);
        assert_eq!(evaluation[0], Object::Boolean(false));
    }
}
