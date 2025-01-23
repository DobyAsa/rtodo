use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Todo {
    pub title: String,
    pub finished: bool
}

impl Todo {
    pub fn new(title: &str) -> Todo {
        Todo {
            title: title.to_string(),
            finished: false
        }
    }
}