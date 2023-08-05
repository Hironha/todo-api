use std::sync::{Arc, Mutex};

use crate::{
    application::functions::todo::{
        CreatePayload, TodoCreator, TodoDeleter, TodoGetter, TodoLister, TodoSetter, UpdatePayload,
    },
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

impl TodoLister for TodoStore {
    fn list(&self) -> Result<Vec<Todo>, String> {
        let store = self.todos.lock().unwrap();
        let todos = store.clone();
        Ok(todos)
    }
}

impl TodoGetter for TodoStore {
    fn get(&self, id: &str) -> Result<Todo, String> {
        let store = self.todos.lock().unwrap();
        store
            .iter()
            .find_map(|t| if t.id == id { Some(t.clone()) } else { None })
            .ok_or(format!("Todo with id {id} not found"))
    }
}

impl TodoDeleter for TodoStore {
    fn delete(&mut self, id: &str) -> Result<Todo, String> {
        let mut store = self.todos.lock().unwrap();

        if let Some(index) = store.iter().position(|t| t.id == id) {
            let deleted_todo = store.remove(index);
            Ok(deleted_todo)
        } else {
            Err(format!("Todo with id {id} not found"))
        }
    }
}

impl TodoSetter for TodoStore {
    fn set(&self, payload: UpdatePayload) -> Result<Todo, String> {
        let mut store = self.todos.lock().unwrap();
        let index = store.iter().position(|t| t.id == payload.id);
        let todo = index
            .and_then(|i| store.get_mut(i))
            .ok_or(format!("todo with id {} not found", payload.id))?;

        todo.description = payload.description;
        todo.todo_at = payload.todo_at;
        todo.title = payload.title.unwrap_or(todo.title.clone());

        Ok(todo.clone())
    }
}
