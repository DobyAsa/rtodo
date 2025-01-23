use std::io::stdout;

use crossterm::{execute, style::Print};

mod todo;
mod table;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    execute!(
        stdout(),
        Print("Hello "),
        Print("Crossterm!\n")
    )?;

    Ok(())
}