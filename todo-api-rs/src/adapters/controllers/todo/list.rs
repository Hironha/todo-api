use crate::adapters::dtos::todo::list::{Item, Output};
use crate::application::functions::todo::{list_todo, List, ListContext, ListError};
use crate::domain::todo::Todo;

#[derive(Debug)]
pub enum RunError {
    Internal,
}

pub struct ListController<S: List> {
    store: S,
}

impl<S: List> ListController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self) -> Result<Output, RunError> {
        let context = ListContext { store: self.store };
        let todos = list_todo(context).await.map_err(|e| match e {
            ListError::StorageAccess => RunError::Internal,
        })?;

        let items: Vec<Item> = todos.into_iter().map(create_item_from_todo).collect();
        Ok(Output {
            count: items.len(),
            items,
        })
    }
}

fn create_item_from_todo(todo: Todo) -> Item {
    Item {
        id: todo.id.as_string(),
        title: todo.title,
        description: todo.description,
        todo_at: todo.todo_at.map(|at| at.ymd()),
        created_at: todo.created_at.rfc3339(),
        updated_at: todo.updated_at.rfc3339(),
    }
}
