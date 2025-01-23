//! Todo module
//! 
//! This module defines the data structure and operations for individual todos, including:
//! - Creating new todos
//! - Getting and modifying todo titles and descriptions
//! - Managing todo statuses (completed, abandoned, etc.)
//! 
//! # Examples
//! 
//! ```
//! use rtodo::table::Todo;
//! 
//! // Create new todo
//! let mut todo = Todo::new("Write code".to_string(), "Complete Todo project".to_string());
//! 
//! // Modify todo
//! todo.modify_title("Refactor code".to_string());
//! todo.modify_description("Optimize code structure".to_string());
//! 
//! // Update status
//! todo.finish(); // Mark as completed
//! todo.unfinish(); // Reset to incomplete
//! todo.forgive(); // Abandon the todo
//! ```

use std::fmt::Display;
use serde::{Deserialize, Serialize};

/// Todo structure
///
/// Contains basic todo information: title, description, and status
///
/// # Fields
///
/// * `title` - Todo title
/// * `description` - Detailed description
/// * `status` - Current status
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
}

/// Todo status enumeration
///
/// Defines all possible todo statuses:
/// * `Unfinished` - Incomplete
/// * `Finished` - Completed
/// * `Forgave` - Abandoned
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TodoStatus {
    Unfinished,
    Finished,
    Forgave,
}

impl Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Unfinished => write!(f, "Unfinished"),
            TodoStatus::Finished => write!(f, "Finished"),
            TodoStatus::Forgave => write!(f, "Abandoned"),
        }
    }
}

impl Todo {
    /// Creates a new todo
    ///
    /// # Arguments
    ///
    /// * `title` - Todo title
    /// * `description` - Todo description
    ///
    /// # Returns
    ///
    /// New `Todo` instance with initial status set to Unfinished
    ///
    /// # Examples
    ///
    /// ```
    /// use rtodo::table::Todo;
    ///
    /// let todo = Todo::new("Learn Rust".to_string(), "Complete Rust tutorial chapter 1".to_string());
    /// assert_eq!(todo.get_title(), "Learn Rust");
    /// ```
    pub fn new(title: String, description: String) -> Todo {
        Todo {
            title,
            description,
            status: TodoStatus::Unfinished,
        }
    }

    /// Gets todo title
    ///
    /// # Returns
    ///
    /// Cloned title string
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Gets todo description
    ///
    /// # Returns
    ///
    /// Cloned description string
    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    /// Gets current status
    ///
    /// # Returns
    ///
    /// Reference to status
    pub fn get_status(&self) -> &TodoStatus {
        &self.status
    }

    /// Modifies todo title
    ///
    /// # Arguments
    ///
    /// * `title` - New title
    pub fn modify_title(&mut self, title: String) {
        self.title = title;
    }

    /// Modifies todo description
    ///
    /// # Arguments
    ///
    /// * `desc` - New description
    pub fn modify_description(&mut self, desc: String) {
        self.description = desc;
    }

    /// Marks todo as completed
    pub fn finish(&mut self) {
        self.status = TodoStatus::Finished
    }

    /// Marks todo as abandoned
    pub fn forgive(&mut self) {
        self.status = TodoStatus::Forgave
    }

    /// Resets todo to incomplete status
    pub fn unfinish(&mut self) {
        self.status = TodoStatus::Unfinished
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} ({})", 
            match self.status {
                TodoStatus::Unfinished => "Unfinished",
                TodoStatus::Finished => "Finished",
                TodoStatus::Forgave => "Abandoned"
            },
            self.title,
            self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let title = "title".to_string();
        let description = "description".to_string();
        let todo = Todo::new(title.clone(), description.clone());
        assert_eq!(todo, Todo {
            title: title,
            description: description,
            status: TodoStatus::Unfinished,
        })
    }

    #[test]
    fn test_modify_title() {
        let title = "title".to_string();
        let description = "description".to_string();
        let mut todo = Todo::new(title.clone(), description.clone());
        assert_eq!(todo.title, title);

        let new_title = String::from("new title");
        todo.modify_title(new_title.clone());
        assert_eq!(todo.title, new_title);
    }

    #[test]
    fn test_modify_description() {
        let title = "title".to_string();
        let description = "description".to_string();
        let mut todo = Todo::new(title.clone(), description.clone());
        assert_eq!(todo.description, description);

        todo.modify_description(description.clone());
        assert_eq!(todo.description, description)
    }
}