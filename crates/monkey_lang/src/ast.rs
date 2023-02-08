use crate::token::Token;

pub(crate) trait Node {
    // TODO: add a default implementation for this
    //  since most nodes have a token associated with them
    //  can assert that by forcing the implementation of the
    //  get_token method
    fn token_literal(&self) -> String;
}
pub(crate) trait Statement: Node {}
pub(crate) trait Expression: Node {}

/// Represents the program as a series of statements
pub(crate) struct Program {
    // TODO: maybe make this private with new method
    pub(crate) statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub(crate) fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            "".to_string()
        } else {
            self.statements[0].token_literal()
        }
    }
}

/// Represents the name of something
#[derive(Clone)]
pub(crate) struct Identifier {
    token: Token,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {}

/// Represents let statements of the form
/// let <identifier> = <expression>;
/// e.g let a = 2;
pub(crate) struct LetStatement {
    pub(crate) token: Token, // weird that we want this, since it's would just be LET
    pub(crate) name: Identifier,
    pub(crate) value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> Self {
        Self { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {}
