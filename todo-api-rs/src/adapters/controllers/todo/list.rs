use crate::adapters::dtos::todo::list::{Output, RunError};
use crate::application::dtos::todo::list::ListTodoError;
use crate::application::functions::todo::{list_todo, ListContext};
use crate::application::repositories::todo::list::List;

pub struct ListController<S: List> {
    store: S,
}

impl<S: List> ListController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self) -> Output {
        let context = ListContext { store: self.store };
        let result = list_todo(context).await.into_result();

        match result {
            Ok(todos) => Output::from_todos(todos),
            Err(err) => Output::err(match err {
                ListTodoError::Internal => RunError::Internal,
            }),
        }
    }
}
