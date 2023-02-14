use crate::token::Token;
use std::fmt::{format, Debug, Display, Formatter};

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

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub(crate) statements: Vec<Statement>,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let block_strings = self
            .statements
            .iter()
            .map(|statement| statement.to_string())
            .collect::<Vec<String>>();
        let block_string = block_strings.join("\n");
        f.write_str(&format!("{{{}}}", block_string.as_str()))
    }
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
    /// Represents a boolean value i.e true or false
    Boolean(bool),
    /// Represents an If block, with optional else
    If {
        condition: Box<Expression>,
        consequence: Block,
        alternative: Option<Block>,
    },
    /// Represents a function definition
    FunctionLiteral {
        parameters: Vec<String>,
        body: Block,
    },
    /// Represents a function call
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Box<Expression>>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(value) => f.write_str(value.as_str()),
            Expression::IntegerLiteral(value) => f.write_str(&format!("{}", value)),
            Expression::Prefix { operator, right } => f.write_str(&format!("({operator}{right})")),
            Expression::Infix {
                left,
                operator,
                right,
            } => f.write_str(&format!("({left} {operator} {right})")),
            Expression::Boolean(value) => f.write_str(&format!("{}", value)),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let if_expression_string = match alternative {
                    Some(alternative) => format!("if({condition}){consequence} else{alternative}"),
                    None => format!("if({condition}){consequence}"),
                };
                f.write_str(if_expression_string.as_str())
            }
            Expression::FunctionLiteral { parameters, body } => {
                let comma_seperated_parameters = parameters.join(", ");
                return f.write_str(&format!("fn({comma_seperated_parameters}){body}"));
            }
            Expression::FunctionCall {
                function,
                arguments,
            } => {
                let comma_seperated_arguments = arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                return f.write_str(&format!("{function}({comma_seperated_arguments})"));
            }
        }
    }
}

/// Represents the program as a series of statements
pub struct Program {
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
        f.write_str(program_string.as_str())
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
