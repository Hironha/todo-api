use crate::adapters::dtos::todo::delete::{Input, Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::application::functions::todo::{
    delete_todo, Delete, DeleteContext, DeleteError, DeletePayload,
};

pub struct DeleteController<S: Delete> {
    store: S,
}

impl<S: Delete> DeleteController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<Input, ParseError>) -> Output {
        let payload = match input.parse() {
            Ok(input) => DeletePayload { id: input.id },
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = DeleteContext { store: self.store };
        let result = delete_todo(ctx, payload).await.map_err(|err| match err {
            DeleteError::NotFound => RunError::NotFound,
            DeleteError::InternalError => RunError::Internal,
        });

        match result {
            Ok(_) => Output::ok(),
            Err(err) => Output::err(err),
        }
    }
}
