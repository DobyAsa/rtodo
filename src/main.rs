use crate::table::{Table, Todo};

mod table;

fn main() {
    let mut table = Table::new();

    table.add_todo(Todo::new("todo".to_string(), "enheng".to_string()));

    println!("{}", table.serialize().unwrap());
}