use crate::ast::{Block, Expression, InfixOperator, PrefixOperator, Program, Statement};
use crate::error::Error;
use crate::object::{EvaluationValue, Object};

// TODO: implement proper error handling
//  by making certain things stricter

/// Evaluates a vector of statements, returning a corresponding vector of objects
/// for each statement
// TODO: this should also return a result
fn eval_program(program: &Program) -> Result<Object, Error> {
    Ok(eval_statements(&program.statements)?.object)
}

/// Same as eval_program but converts object to string after
pub fn eval_program_string_output(program: &Program) -> Result<String, Error> {
    let evaluation = eval_program(program)?;
    Ok(evaluation.to_string())
}

/// Evaluate satements
fn eval_statements(statements: &Vec<Statement>) -> Result<EvaluationValue, Error> {
    let mut evaluation: EvaluationValue = Object::Null.into();
    for statement in statements {
        match statement {
            Statement::Expression(expr) => {
                evaluation = eval_expression(expr)?;
                if evaluation.is_return_value {
                    break;
                }
            }
            Statement::Return { return_value } => {
                evaluation = eval_expression(return_value)?;
                evaluation.is_return_value = true;
                break;
            }
            _ => evaluation = Object::Null.into(),
        }
    }
    Ok(evaluation)
}

/// Evaluates an expression
fn eval_expression(expression: &Expression) -> Result<EvaluationValue, Error> {
    match expression {
        Expression::IntegerLiteral(value) => Ok(Object::Integer(value.to_owned()).into()),
        Expression::Boolean(bool) => Ok(Object::Boolean(bool.to_owned()).into()),
        Expression::Prefix { operator, right } => {
            let right_eval = eval_expression(right)?;
            eval_prefix_expression(operator, right_eval.object)
        }
        Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left_eval = eval_expression(left)?;
            let right_eval = eval_expression(right)?;
            eval_infix_expression(operator, left_eval.object, right_eval.object)
        }
        Expression::If {
            condition,
            consequence,
            alternative,
        } => eval_if_expression(condition, consequence, alternative),
        _ => todo!(),
    }
}

/// Evaluates a prefix expression
fn eval_prefix_expression(
    operator: &PrefixOperator,
    right: Object,
) -> Result<EvaluationValue, Error> {
    match operator {
        PrefixOperator::BANG => Ok(eval_bang_prefix_operator(right)),
        PrefixOperator::NEGATE => eval_minus_prefix_operator(right),
    }
}

/// Evaluates the bang operator on an object
fn eval_bang_prefix_operator(obj: Object) -> EvaluationValue {
    match obj {
        Object::Boolean(val) => Object::Boolean(!val).into(),
        Object::Integer(val) => {
            if val == 0 {
                // 0 means false, hence this is converted to Object::Boolean(false)
                // applying the bang operator we get Object::Boolean(true)
                Object::Boolean(true).into()
            } else {
                Object::Boolean(false).into()
            }
        }
        Object::Null => {
            // null by default represents false, so we return true
            Object::Boolean(true).into()
        }
    }
}

/// Evaluates the negation operator on an object
fn eval_minus_prefix_operator(obj: Object) -> Result<EvaluationValue, Error> {
    match obj {
        Object::Integer(val) => Ok(Object::Integer(-1 * val).into()),
        _ => Err(Error::UnknownOperator(format!("-{}", obj.to_type_string()))),
    }
}

/// Evaluates an infix expression
fn eval_infix_expression(
    operator: &InfixOperator,
    left: Object,
    right: Object,
) -> Result<EvaluationValue, Error> {
    match (left, right) {
        (Object::Integer(a), Object::Integer(b)) => {
            Ok(eval_integer_infix_expression(operator, a, b))
        }
        (Object::Boolean(a), Object::Boolean(b)) => eval_boolean_infix_expression(operator, a, b),
        (left, right) => Err(Error::TypeMismatch {
            left: left.to_type_string(),
            operator: operator.to_string(),
            right: right.to_type_string(),
        }),
    }
}

/// Performs infix operations on integer types
fn eval_integer_infix_expression(
    operator: &InfixOperator,
    left: i64,
    right: i64,
) -> EvaluationValue {
    match operator {
        InfixOperator::PLUS => Object::Integer(left + right).into(),
        InfixOperator::MINUS => Object::Integer(left - right).into(),
        InfixOperator::MULTIPLY => Object::Integer(left * right).into(),
        InfixOperator::DIVIDE => Object::Integer(left / right).into(),
        InfixOperator::LESSTHAN => Object::Boolean(left < right).into(),
        InfixOperator::GREATERTHAN => Object::Boolean(left > right).into(),
        InfixOperator::EQUAL => Object::Boolean(left == right).into(),
        InfixOperator::NOTEQUAL => Object::Boolean(left != right).into(),
    }
}

/// Performs infix operations on boolean types
fn eval_boolean_infix_expression(
    operator: &InfixOperator,
    left: bool,
    right: bool,
) -> Result<EvaluationValue, Error> {
    // TODO: add support for greater and less than
    match operator {
        InfixOperator::EQUAL => Ok(Object::Boolean(left == right).into()),
        InfixOperator::NOTEQUAL => Ok(Object::Boolean(left != right).into()),
        _ => Err(Error::UnknownOperator(format!(
            "BOOLEAN {} BOOLEAN",
            operator
        ))),
    }
}

/// Evaluates an if expression
fn eval_if_expression(
    condition: &Box<Expression>,
    consequence: &Block,
    alternative: &Option<Block>,
) -> Result<EvaluationValue, Error> {
    let condition_eval = eval_expression(condition)?.object;
    if condition_eval.is_truthy() {
        eval_block(consequence)
    } else if let Some(alternative) = alternative {
        eval_block(alternative)
    } else {
        Ok(Object::Null.into())
    }
}

/// Evaluates a block and returns the evaluation of the last statement
/// in the block
fn eval_block(block: &Block) -> Result<EvaluationValue, Error> {
    eval_statements(&block.statements)
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::error::Error;
    use crate::evaluator::{eval_expression, eval_program};
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    fn parse_and_eval_program_possible_error(input: &str) -> Result<Object, Error> {
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        eval_program(&program)
    }

    fn parse_and_eval_program(input: &str) -> Object {
        parse_and_eval_program_possible_error(input).unwrap()
    }

    #[test]
    fn eval_integer_expression() {
        let input = "5";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(5));

        let input = "10";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "-5";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(-5));

        let input = "-10";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(-10));
    }

    #[test]
    fn eval_boolean_expression() {
        let input = "true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));
    }

    #[test]
    fn eval_bang_operator() {
        let input = "!true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "!false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        // ???
        let input = "!5";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "!!true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "!!false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "!!5";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));
    }

    #[test]
    fn eval_integer_infix_expression() {
        let input = "5 + 5 + 5 + 5 - 10";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "2 * 2 * 2 * 2 * 2";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(32));

        let input = "2 * (5 + 10)";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(30));

        let input = "(5 + 10 * 2 + 15 / 3) * 2 + -10";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(50));

        let input = "1 < 2";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "1 > 2";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "1 < 1";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "1 > 1";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "1 == 1";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "1 != 1";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "1 == 2";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "1 != 2";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));
    }

    #[test]
    fn eval_boolean_infix_expression() {
        let input = "true == true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "false == false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "true == false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "true != false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "false != true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "(1 < 2) == false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));

        let input = "(1 < 2) == true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "(1 > 2) == false";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(true));

        let input = "(1 > 2) == true";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Boolean(false));
    }

    #[test]
    fn eval_if_expression() {
        let input = "if (true) { 10 }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "if (false) { 10 }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Null);

        let input = "if (1) { 10 }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "if (1 < 2) { 10 }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "if (1 > 2) { 10 }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Null);

        let input = "if (1 < 2) { 10 } else { 20 }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "if (1 > 2) { 10 } else { 20 }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(20));
    }

    #[test]
    fn eval_return_expression() {
        let input = "return 10;";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "return 10; 9;";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "return 2 * 5; 9;";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "9; return 2 * 5; 9;";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));

        let input = "\
        if (10 > 1) {\
            if (10 > 1) {\
                return 10;\
            }\
            return 1;\
         }";
        let evaluation = parse_and_eval_program(input);
        assert_eq!(evaluation, Object::Integer(10));
    }

    #[test]
    fn error_handling() {
        let input = "5 + true;";
        let evaluation = parse_and_eval_program_possible_error(input);
        assert!(evaluation.is_err());
        assert_eq!(
            evaluation,
            Err(Error::TypeMismatch {
                left: "INTEGER".to_string(),
                operator: "+".to_string(),
                right: "BOOLEAN".to_string()
            })
        );

        let input = "5 + true; 5;";
        let evaluation = parse_and_eval_program_possible_error(input);
        assert!(evaluation.is_err());
        assert_eq!(
            evaluation,
            Err(Error::TypeMismatch {
                left: "INTEGER".to_string(),
                operator: "+".to_string(),
                right: "BOOLEAN".to_string()
            })
        );

        let input = "-true;";
        let evaluation = parse_and_eval_program_possible_error(input);
        assert!(evaluation.is_err());
        assert_eq!(
            evaluation,
            Err(Error::UnknownOperator("-BOOLEAN".to_string()))
        );

        let input = "true + false;";
        let evaluation = parse_and_eval_program_possible_error(input);
        assert!(evaluation.is_err());
        assert_eq!(
            evaluation,
            Err(Error::UnknownOperator("BOOLEAN + BOOLEAN".to_string()))
        );

        let input = "5; true + false; 5;";
        let evaluation = parse_and_eval_program_possible_error(input);
        assert!(evaluation.is_err());
        assert_eq!(
            evaluation,
            Err(Error::UnknownOperator("BOOLEAN + BOOLEAN".to_string()))
        );

        let input = "if (10 > 1) { true + false; }";
        let evaluation = parse_and_eval_program_possible_error(input);
        assert!(evaluation.is_err());
        assert_eq!(
            evaluation,
            Err(Error::UnknownOperator("BOOLEAN + BOOLEAN".to_string()))
        );

        let input = "\
        if (10 > 1) {\
            if (10 > 1) {\
                true + false;\
            }\
            return 1;\
         }";
        let evaluation = parse_and_eval_program_possible_error(input);
        assert!(evaluation.is_err());
        assert_eq!(
            evaluation,
            Err(Error::UnknownOperator("BOOLEAN + BOOLEAN".to_string()))
        );
    }
}
