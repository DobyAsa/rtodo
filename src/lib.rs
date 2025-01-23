//! rtodo is a command-line todo management tool
//!
//! This library provides basic todo management features including:
//! - Adding new todo items
//! - Listing all todos
//! - Removing todos
//! - Marking todos as complete
//! - Abandoning todos
//!
//! All todo data is persisted in `todo.json` file.

pub mod table;

pub use table::{Table, Todo};

use clap::Parser;

/// Command-line argument parsing structure
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Subcommand
    #[command(subcommand)]
    command: Command,
}

/// Available command list
#[derive(clap::Subcommand)]
pub enum Command {
    /// Add a new todo item
    #[clap(about = "Add a new todo item")]
    Add,
    /// List all todo items
    #[clap(about = "List all todos")]
    List,
    /// Remove specified todo item
    #[clap(about = "Remove a todo item")]
    Remove,
    /// Mark todo item as complete
    #[clap(about = "Complete a todo item")]
    Finish,
    /// Mark todo item as abandoned
    #[clap(about = "Abandon a todo item")]
    Forgive,
}

/// Main entry function for the program
///
/// Parses command-line arguments and executes corresponding operations.
/// All changes are persisted to todo.json file.
///
/// # Errors
///
/// Returns `std::io::Error` on IO errors
pub fn run() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let mut table = init_table();
    let result = match args.command {
        Command::Add => add(&mut table),
        Command::List => list(&table),
        Command::Remove => remove(&mut table),
        Command::Finish => finish(&mut table),
        Command::Forgive => forgive(&mut table),
    };
    
    // Save changes to file
    if let Ok(json) = serde_json::to_string_pretty(&table) {
        std::fs::write("todo.json", json)?;
    }
    
    result
}

/// Initialize todo table
///
/// Attempts to load existing todos from todo.json file.
/// Creates new empty table if file doesn't exist or is unreadable.
///
/// # Returns
///
/// Initialized `Table` instance
fn init_table() -> Table {
    if let Ok(file) = std::fs::File::open("todo.json") {
        let table: Table = serde_json::from_reader(file).unwrap();
        return table;
    }
    Table::new()
}

/// Add new todo item
///
/// Reads title and description from stdin to create new todo.
///
/// # Arguments
///
/// * `table` - Mutable reference to todo table
///
/// # Errors
///
/// Returns `std::io::Error` on stdin read failure
fn add(table: &mut Table) -> Result<(), std::io::Error> {
    println!("Enter todo title:");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title)?;
    let title = title.trim().to_string();

    println!("Enter todo description:");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description)?;
    let description = description.trim().to_string();

    let todo = Todo::new(title, description);
    let id = table.add_todo(todo);
    println!("✅ Successfully added todo #{}", id);
    Ok(())
}

/// List all todo items
///
/// # Arguments
///
/// * `table` - Reference to todo table
///
/// # Errors
///
/// Returns `std::io::Error` on stdout write failure
fn list(table: &Table) -> Result<(), std::io::Error> {
    println!("{}", table);
    Ok(())
}

/// Remove specified todo item
///
/// Reads todo ID from stdin and removes it from the table.
///
/// # Arguments
///
/// * `table` - Mutable reference to todo table
///
/// # Errors
///
/// Returns `std::io::Error` on stdin read failure or invalid ID format
fn remove(table: &mut Table) -> Result<(), std::io::Error> {
    println!("Enter todo ID to remove:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let id = input.trim().parse::<usize>().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid ID")
    })?;

    if let Some(_) = table.remove_todo_by_id(id) {
        println!("✅ Successfully removed todo #{}", id);
    } else {
        println!("❌ Todo #{} not found", id);
    }
    Ok(())
}

/// Mark todo item as complete
///
/// Reads todo ID from stdin and updates its status to completed.
///
/// # Arguments
///
/// * `table` - Mutable reference to todo table
///
/// # Errors
///
/// Returns `std::io::Error` on stdin read failure or invalid ID format
fn finish(table: &mut Table) -> Result<(), std::io::Error> {
    println!("Enter todo ID to complete:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let id = input.trim().parse::<usize>().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid ID")
    })?;

    if let Some(todo) = table.get_todo_by_id(id) {
        todo.finish();
        println!("✅ Marked todo #{} as completed", id);
    } else {
        println!("❌ Todo #{} not found", id);
    }
    Ok(())
}

/// Mark todo item as abandoned
///
/// Reads todo ID from stdin and updates its status to abandoned.
///
/// # Arguments
///
/// * `table` - Mutable reference to todo table
///
/// # Errors
///
/// Returns `std::io::Error` on stdin read failure or invalid ID format
fn forgive(table: &mut Table) -> Result<(), std::io::Error> {
    println!("Enter todo ID to abandon:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let id = input.trim().parse::<usize>().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid ID")
    })?;

    if let Some(todo) = table.get_todo_by_id(id) {
        todo.forgive();
        println!("✅ Marked todo #{} as abandoned", id);
    } else {
        println!("❌ Todo #{} not found", id);
    }
    Ok(())
}