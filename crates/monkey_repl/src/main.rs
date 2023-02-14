use monkey_lang::evaluator::eval_program_string_output;
use monkey_lang::lexer::Lexer;
use monkey_lang::parser::Parser;
use monkey_lang::token::TokenType;
use std::io;
use std::io::Write;

// TODO: if you encounter a parse error pretty print it
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        // TODO: eventually reduce the information the repl has access to
        // build a lexer from this and then call parser
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        // TODO: maybe not parser a program but a statement
        let program = parser.parse_program().unwrap();
        // TODO: should we be evaluating statements instead??
        let evaluation = eval_program_string_output(&program);

        println!("{}", evaluation[0]);

        input.clear();
    }
}
