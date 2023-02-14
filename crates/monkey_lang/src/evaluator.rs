use crate::ast::Expression;
use crate::object::Object;

// TODO: does expression have to be moved??
fn eval_expression(expression: Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(value) => Object::Integer(value),
        _ => Object::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::evaluator::eval_expression;
    use crate::object::Object;

    #[test]
    fn eval_integer_expression() {
        let int_5 = Expression::IntegerLiteral(5);
        assert_eq!(eval_expression(int_5), Object::Integer(5));
    }
}
