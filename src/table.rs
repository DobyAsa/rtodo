//! Todo table module
//!
//! This module provides centralized todo management features including:
//! - Adding new todos
//! - Removing todos
//! - Modifying todos
//! - Retrieving todos
//! - Serializing todo tables
//!
//! # Examples
//!
//! ```
//! use rtodo::table::Table;
//! use rtodo::table::Todo;
//!
//! // Create new todo table
//! let mut table = Table::new();
//!
//! // Add todo
//! let todo = Todo::new("Complete homework".to_string(), "Finish math assignment".to_string());
//! let id = table.add_todo(todo);
//!
//! // Modify todo
//! let new_todo = Todo::new("Finish all homework".to_string(), "Complete both math and English assignments".to_string());
//! table.modify_todo_by_id(id, new_todo).unwrap();
//!
//! // Get todo
//! if let Some(todo) = table.get_todo_by_id(id) {
//!     todo.finish(); // Mark as complete
//! }
//!
//! // Remove todo
//! table.remove_todo_by_id(id);
//!
//! // Serialize table
//! if let Some(json) = table.serialize() {
//!     println!("{}", json);
//! }
//! ```

#![allow(unused)]

mod todo;

pub use crate::table::todo::Todo;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

/// Todo table structure
///
/// Uses `BTreeMap` for storage to maintain ID ordering
/// 
/// # Fields
///
/// * `todos` - Map storing all todos (ID as key)
/// * `next_id` - Next todo ID counter
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    todos: BTreeMap<usize, Todo>,
    next_id: usize,
}

impl Table {
    /// Creates a new empty table
    ///
    /// # Returns
    ///
    /// New `Table` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use rtodo::table::Table;
    ///
    /// let table = Table::new();
    /// ```
    pub fn new() -> Table {
        Table {
            todos: BTreeMap::new(),
            next_id: 0,
        }
    }

    /// Adds a new todo to the table
    ///
    /// # Arguments
    ///
    /// * `todo` - Todo item to add
    ///
    /// # Returns
    ///
    /// ID of the newly added todo
    ///
    /// # Examples
    ///
    /// ```
    /// use rtodo::table::{Table, Todo};
    ///
    /// let mut table = Table::new();
    /// let todo = Todo::new("Test".to_string(), "This is a test".to_string());
    /// let id = table.add_todo(todo);
    /// ```
    pub fn add_todo(&mut self, todo: Todo) -> usize {
        self.todos.insert(self.next_id, todo);
        self.next_id += 1;
        self.next_id - 1
    }

    /// Removes a todo by ID
    ///
    /// # Arguments
    ///
    /// * `id` - ID of todo to remove
    ///
    /// # Returns
    ///
    /// Returns `Some(Todo)` if found and removed, otherwise `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use rtodo::table::{Table, Todo};
    ///
    /// let mut table = Table::new();
    /// let todo = Todo::new("Test".to_string(), "This is a test".to_string());
    /// let id = table.add_todo(todo);
    ///
    /// if let Some(removed_todo) = table.remove_todo_by_id(id) {
    ///     println!("Successfully removed todo");
    /// }
    /// ```
    pub fn remove_todo_by_id(&mut self, id: usize) -> Option<Todo> {
        self.todos.remove(&id)
    }

    /// Modifies a todo by ID
    ///
    /// # Arguments
    ///
    /// * `id` - ID of todo to modify
    /// * `new_todo` - New todo content
    ///
    /// # Returns
    ///
    /// Returns `Ok(&Todo)` on success, `Err` with message on failure
    ///
    /// # Examples
    ///
    /// ```
    /// use rtodo::table::{Table, Todo};
    ///
    /// let mut table = Table::new();
    /// let todo = Todo::new("Old title".to_string(), "Old description".to_string());
    /// let id = table.add_todo(todo);
    ///
    /// let new_todo = Todo::new("New title".to_string(), "New description".to_string());
    /// match table.modify_todo_by_id(id, new_todo) {
    ///     Ok(_) => println!("Modification successful"),
    ///     Err(e) => println!("Modification failed: {}", e),
    /// }
    /// ```
    pub fn modify_todo_by_id(&mut self, id: usize, new_todo: Todo) -> Result<&Todo, String> {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.modify_title(new_todo.title);
            todo.modify_description(new_todo.description);
            Ok(todo)
        } else {
            Err("Todo not found".to_string())
        }
    }

    /// Gets a mutable reference to a todo by ID
    ///
    /// # Arguments
    ///
    /// * `id` - ID of todo to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut Todo)` if found, otherwise `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use rtodo::table::{Table, Todo};
    ///
    /// let mut table = Table::new();
    /// let todo = Todo::new("Test".to_string(), "This is a test".to_string());
    /// let id = table.add_todo(todo);
    ///
    /// if let Some(todo) = table.get_todo_by_id(id) {
    ///     todo.finish(); // Mark todo as complete
    /// }
    /// ```
    pub fn get_todo_by_id(&mut self, id: usize) -> Option<&mut Todo> {
        self.todos.get_mut(&id)
    }

    /// Serializes the table to JSON string
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` on success, `None` on failure
    ///
    /// # Examples
    ///
    /// ```
    /// use rtodo::table::{Table, Todo};
    ///
    /// let mut table = Table::new();
    /// let todo = Todo::new("Test".to_string(), "This is a test".to_string());
    /// table.add_todo(todo);
    ///
    /// if let Some(json) = table.serialize() {
    ///     println!("Serialized result: {}", json);
    /// }
    /// ```
    pub fn serialize(&self) -> Option<String> {
        match serde_json::to_string_pretty(self) {
            Ok(string) => Some(string),
            Err(_) => None,
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.todos.is_empty() {
            return writeln!(f, "📝 Todo list is empty");
        }

        writeln!(f, "📝 Todo list:")?;
        for (id, todo) in self.todos.iter() {
            writeln!(f, "{:>3}. {}", id, todo)?;
        }
        Ok(())
    }
}