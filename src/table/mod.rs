mod todo;

pub use crate::table::todo::Todo;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug,PartialEq,Serialize)]
pub struct Table {
    todos: HashMap<usize, Todo>,
    next_id: usize,
}

impl Table {
    pub fn new() -> Table {
        Table {
            todos: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_todo(&mut self, todo: Todo) -> usize {
        self.todos.insert(self.next_id, todo);
        self.next_id += 1;
        self.next_id - 1
    }

    fn remove_todo_by_id(&mut self, id: usize) {
        self.todos.remove(&id);
    }

    pub fn modify_todo(&mut self, id: usize, new_todo: Todo) -> Result<&Todo, String> {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.modify_title(new_todo.title);
            todo.modify_description(new_todo.description);
            Ok(todo)
        } else {
            Err("Not found".to_string())
        }
    }

    pub fn get_todos(&self, id: usize) -> Option<&Todo> {
        self.todos.get(&id)
    }

    pub fn serialize(&self) -> Option<String> {
        match serde_json::to_string_pretty(self) {
            Ok(string) => Some(string),
            Err(_) => None,
        }
    }
}