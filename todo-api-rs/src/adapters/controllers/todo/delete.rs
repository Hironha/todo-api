use crate::adapters::dtos::todo::delete::{Input, Output, ParseError};
use crate::adapters::dtos::ParsableInput;
use crate::application::functions::todo::{
    delete_todo, Delete, DeleteContext, DeleteError, DeletePayload,
};

#[derive(Debug)]
pub enum RunError {
    Validation(ParseError),
    NotFound,
    Internal,
}

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

    pub async fn run(self) -> Result<Output, RunError> {
        let input = self.input.parse().map_err(RunError::Validation)?;
        let payload = DeletePayload { id: input.id };

        let ctx = DeleteContext { store: self.store };
        delete_todo(ctx, payload).await.map_err(|err| match err {
            DeleteError::NotFound => RunError::NotFound,
            DeleteError::InternalError => RunError::Internal,
        })?;

        Ok(Output::new())
    }
}
