use std::collections::HashMap;

use serde::Serialize;

use crate::todo::Todo;

#[derive(Serialize)]
pub struct Table {
    todos: HashMap<String, Todo>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            todos: HashMap::new(),
        }
    }

    pub fn add_todo(&mut self, title: &str) {
        self.todos.insert(title.to_string(), Todo::new(title));
    }

    pub fn remove_todo(&mut self, title: &str) {
        self.todos.remove(title);
    }

    pub fn modify_todo(&mut self, title: &str, new_title: &str) {
        let v = self.todos.remove(title).unwrap_or(Todo::new(title));
        self.todos.insert(new_title.to_string(), v);
    }

    pub fn get_todos(&self, title: &str) -> Option<&Todo> {
        self.todos.get(title)
    }

    pub fn serialize(&self) -> Option<String> {
        match serde_json::to_string_pretty(self) {
            Ok(string) => Some(string),
            Err(_) => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table_is_empty() {
        let table = Table::new();
        assert!(table.todos.is_empty());
    }

    #[test]
    fn test_add_todo() {
        let mut table = Table::new();
        table.add_todo("test");
        assert_eq!(table.todos.len(), 1);
        let todo = table.todos.get("test").unwrap();
        assert_eq!(todo.title, "test");
        assert!(!todo.finished);
    }

    #[test]
    fn test_add_duplicate_todo_overrides() {
        let mut table = Table::new();
        table.add_todo("test");
        table.add_todo("test");
        assert_eq!(table.todos.len(), 1);
    }

    #[test]
    fn test_remove_existing_todo() {
        let mut table = Table::new();
        table.add_todo("test");
        table.remove_todo("test");
        assert!(table.todos.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_todo() {
        let mut table = Table::new();
        table.add_todo("test");
        table.remove_todo("nonexistent");
        assert_eq!(table.todos.len(), 1);
    }

    #[test]
    fn test_modify_existing_todo() {
        let mut table = Table::new();
        table.add_todo("old_title");
        
        // Modify title and check preservation of state
        if let Some(todo) = table.todos.get_mut("old_title") {
            todo.finished = true;
        }
        
        table.modify_todo("old_title", "new_title");
        
        assert!(!table.todos.contains_key("old_title"));
        let modified_todo = table.todos.get("new_title").unwrap();
        assert!(modified_todo.finished);
    }

    #[test]
    fn test_modify_nonexistent_todo_creates_new() {
        let mut table = Table::new();
        table.modify_todo("nonexistent", "new_title");
        
        assert_eq!(table.todos.len(), 1);
        let todo = table.todos.get("new_title").unwrap();
        assert_eq!(todo.title, "nonexistent"); // 注意：根据当前实现，这会保留原始标题
        assert!(!todo.finished);
    }

    #[test]
    fn test_get_existing_todo() {
        let mut table = Table::new();
        table.add_todo("test");
        assert!(table.get_todos("test").is_some());
    }

    #[test]
    fn test_get_nonexistent_todo() {
        let table = Table::new();
        assert!(table.get_todos("test").is_none());
    }

    #[test]
    fn test_todo_initialization() {
        let todo = Todo::new("test");
        assert_eq!(todo.title, "test");
        assert!(!todo.finished);
    }

    #[test]
    fn test_serialize_table() {
        let mut table = Table::new();
        table.add_todo("todo1");
        table.add_todo("todo2");
        
        // 测试序列化不会panic
        let json = serde_json::to_string(&table).unwrap();
        assert!(json.contains("todo1"));
        assert!(json.contains("todo2"));
    }

    #[test]
    fn test_serialize_todo() {
        let todo = Todo::new("test");
        let json = serde_json::to_string(&todo).unwrap();
        assert!(json.contains(r#""title":"test""#));
        assert!(json.contains(r#""finished":false"#));
    }
}


