use std::io::{stdin, stdout};

use crossterm::{execute, style::PrintStyledContent, style::Stylize};

use crate::table::{Table, Todo};

mod table;

fn main() -> Result<(), std::io::Error> {
    let mut table = Table::new();

    table.add_todo(Todo::new("todo".to_string(), "enheng".to_string()));

    Ok(())
}
