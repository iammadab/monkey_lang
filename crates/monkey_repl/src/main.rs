use monkey_lang::lexer::Lexer;
use monkey_lang::token::TokenType;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        // build a lexer from this and then call parser
        let mut lexer = Lexer::new(input.chars());

        let mut next_token = lexer.next_token();
        while next_token != None {
            next_token = lexer.next_token();
        }

        input.clear();
    }
}
