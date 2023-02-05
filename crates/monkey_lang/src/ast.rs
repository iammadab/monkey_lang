trait Node {
    fn token_literal(&self) -> String;
}
trait Statement: Node {}
trait Expression: Node {}

struct Program<S: Statement> {
    statements: Vec<S>
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