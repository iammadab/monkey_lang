use crate::token::Token;
use std::fmt::{Debug, Display, Formatter};
use thiserror::__private::DisplayAsDisplay;

/// Enum representing the different type of statements we handle
#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    /// Represents let statements of the form
    /// let <identifier> = <expression>;
    /// e.g let a = 2;
    Let { name: String, value: Expression },
    /// Represents statements of the form
    /// return <expression>;
    /// e.g return 2 + 2;
    Return { return_value: Expression },
    /// Wrapper for an expression
    Expression(Expression),
}

/// Enum representing the different type of expressions we handle
#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    /// Represents the name of something
    Identifier(String),
    /// Represents an integer
    IntegerLiteral(i64),
    /// Holds a prefix expression of the form
    /// <prefix><expression>
    /// e.g. -10 where - is the operator and 10 is the right expression
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    /// Hods an infix expression of the form
    /// <expression><operator><expression>
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

/// Represents the program as a series of statements
pub(crate) struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl Program {
    pub(crate) fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let program_strings = self
            .statements
            .iter()
            .map(|statement| statement.to_string())
            .collect::<Vec<String>>();
        let program_string = program_strings.join("\n");
        return f.write_str(program_string.as_str());
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let { name, value } => {
                let statement = format!("let {name} = {value};");
                f.write_str(&statement)
            }
            Statement::Return { return_value } => f.write_str(&format!("return {return_value};")),
            Statement::Expression(expression) => f.write_str(&format!("{expression};")),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(value) => f.write_str(value.as_str()),
            Expression::IntegerLiteral(value) => f.write_str(&format!("{}", value)),
            Expression::Prefix { operator, right } => f.write_str(&format!("{operator}{right}")),
            Expression::Infix {
                left,
                operator,
                right,
            } => f.write_str(&format!("{left}{operator}{right}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Program, Statement};
    use std::fmt::format;

    #[test]
    fn ast_as_string() {
        let mut program = Program::new();
        program.statements.push(Statement::Let {
            name: "my_var".to_string(),
            value: Expression::Identifier("another_var".to_string()),
        });
        program.statements.push(Statement::Return {
            return_value: Expression::Identifier("my_var".to_string()),
        });
        assert_eq!(
            program.to_string(),
            "let my_var = another_var;\n\
            return my_var;"
        );
    }
}
