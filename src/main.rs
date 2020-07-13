use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    eprintln!("Waiting for input. Ctrl+D will flush to standard out");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    io::stdout().write_all(buffer.as_bytes())?;

    Ok(())
}