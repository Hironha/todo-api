use crate::adapters::dtos::todo::delete::{Output, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::delete::{DeleteTodoError, DeleteTodoInput};
use crate::application::functions::todo::delete::{delete_todo, DeleteTodoContext};
use crate::application::repositories::todo::delete::Delete;

pub struct DeleteController<S: Delete> {
    store: S,
}

impl<S: Delete> DeleteController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(&self, input: impl Parse<DeleteTodoInput, ParseError>) -> Output {
        let id = match input.parse() {
            Ok(id) => id,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let ctx = DeleteTodoContext::new(&self.store);
        match delete_todo(ctx, id).await.into_result() {
            Ok(_) => Output::ok(),
            Err(err) => Output::err(match err {
                DeleteTodoError::NotFound => RunError::TodoNotFound,
                DeleteTodoError::Internal => RunError::Internal,
            }),
        }
    }
}
