use std::io::{self, Write};

mod ast;
mod parser;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    
    while !buffer.eq_ignore_ascii_case("quit") {
        stdout.write(b"> ")?;
        stdout.flush()?;
        
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        
        stdout.write(buffer.as_bytes())?;
    };
    Ok(())
}
