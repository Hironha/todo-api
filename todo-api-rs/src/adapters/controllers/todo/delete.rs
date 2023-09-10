use crate::adapters::dtos::todo::delete::{Input, Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::application::functions::todo::{
    delete_todo, Delete, DeleteContext, DeleteError, DeletePayload,
};

pub struct DeleteController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Delete,
{
    input: I,
    store: S,
}

impl<I, S> DeleteController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Delete,
{
    pub const fn new(input: I, store: S) -> Self {
        Self { input, store }
    }

    pub async fn run(self) -> Output {
        let payload = match self.input.parse() {
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
