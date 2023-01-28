use std::io;
use std::io::Write;
use monkey_lang::lexer::Lexer;
use monkey_lang::token::TokenType;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        // build a lexer from this and then call parser
        let mut lexer = Lexer::new(input.chars());

        let mut next_token = lexer.next_token();
        while !matches!(next_token.variant, TokenType::EOF{..}) {
            dbg!(&next_token);
            next_token = lexer.next_token();
        }

        input.clear();
    }
}
