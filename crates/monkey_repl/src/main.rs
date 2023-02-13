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

        // build a lexer from this and then call parser
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        println!("{}", program);

        input.clear();
    }
}
