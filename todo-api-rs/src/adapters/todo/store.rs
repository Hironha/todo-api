use std::sync::{Arc, Mutex};

use crate::{
    application::functions::todo::{CreatePayload, TodoCreator},
    domain::todo::Todo,
};

#[derive(Clone)]
pub struct TodoStore {
    todos: Arc<Mutex<Vec<Todo>>>,
}

impl TodoStore {
    pub fn new() -> Self {
        Self {
            todos: Arc::default(),
        }
    }
}

impl TodoCreator for TodoStore {
    fn create(&mut self, payload: CreatePayload) -> Result<Todo, String> {
        let mut store = self.todos.lock().unwrap();
        let todo = Todo {
            id: "id".to_string(),
            title: payload.title,
            description: payload.description,
            todo_at: payload.todo_at,
            created_at: "created_at".to_string(),
            updated_at: "updated_at".to_string(),
        };
        store.push(todo.clone());
        Ok(todo)
    }
}

// impl TodoLister for TodoStore {
//     fn list(&self) -> Result<Vec<Todo>, String> {
//         let store = self.todos.lock().unwrap();
//         let todos = store.clone();
//         Ok(todos)
//     }
// }
