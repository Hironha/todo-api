use crate::adapters::dtos::todo::delete::{Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::application::functions::todo::{delete_todo, DeleteContext, DeleteTodoError};
use crate::application::repositories::todo::delete::Delete;

pub struct DeleteController<S: Delete> {
    store: S,
}

impl<S: Delete> DeleteController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<String, ParseError>) -> Output {
        let id = match input.parse() {
            Ok(id) => id,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let ctx = DeleteContext { store: self.store };
        let result = delete_todo(ctx, id).await.map_err(|err| match err {
            DeleteTodoError::InvalidId => RunError::InvalidId,
            DeleteTodoError::NotFound => RunError::TodoNotFound,
            DeleteTodoError::Internal => RunError::Internal,
        });

        match result {
            Ok(_) => Output::ok(),
            Err(err) => Output::err(err),
        }
    }
}
