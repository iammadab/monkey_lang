use crate::token::Token;

trait Node {
    // TODO: add a default implementation for this
    //  since most nodes have a token associated with them
    //  can assert that by forcing the implementation of the
    //  get_token method
    fn token_literal(&self) -> String;
}
trait Statement: Node {}
trait Expression: Node {}

/// Represents the program as a series of statements
struct Program<S: Statement> {
    statements: Vec<S>,
}

impl<S: Statement> Node for Program<S> {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            "".to_string()
        } else {
            self.statements[0].token_literal()
        }
    }
}

/// Represents the name of something
struct Identifier {
    token: Token,
    value: String,
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
struct LetStatement<E: Expression> {
    token: Token, // weird that we want this, since it's would just be LET
    name: Identifier,
    value: E,
}

impl<E: Expression> Node for LetStatement<E> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl<E: Expression> Statement for LetStatement<E> {}
