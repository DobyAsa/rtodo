use serde::Serialize;

#[derive(Debug,PartialEq,Serialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub status: TodoStatus
}

#[derive(Debug,PartialEq,Serialize)]
pub enum TodoStatus {
    Unfinished,
    Finished,
    Forgave,
}

impl Todo {
    pub fn new(title: String, description: String) -> Todo {
        Todo {
            title,
            description,
            status: TodoStatus::Unfinished,
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_status(&self) -> &TodoStatus {
        &self.status
    }

    pub fn modify_title(&mut self, title: String) {
        self.title = title.to_string();
    }

    pub fn modify_description(&mut self, desc: String) {
        self.description = desc.to_string();
    }

    pub fn finish(&mut self) {
        self.status = TodoStatus::Finished
    }

    pub fn forgive(&mut self) {
        self.status = TodoStatus::Forgave
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