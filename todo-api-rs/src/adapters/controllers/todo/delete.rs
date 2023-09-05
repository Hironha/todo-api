use crate::adapters::dtos::todo::delete::{Input, Output};
use crate::application::functions::todo::{
    delete_todo, Delete, DeleteContext, DeleteError, DeletePayload,
};

#[derive(Debug)]
pub enum RunError {
    NotFound,
    Internal,
}

pub struct DeleteController<S: Delete> {
    store: S,
}

impl<S: Delete> DeleteController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S: Delete> DeleteController<S> {
    pub async fn run(self, input: Input) -> Result<Output, RunError> {
        let ctx = DeleteContext { store: self.store };
        let payload = DeletePayload { id: input.id };

        delete_todo(ctx, payload).await.map_err(|err| match err {
            DeleteError::NotFound => RunError::NotFound,
            DeleteError::InternalError => RunError::Internal,
        })?;

        Ok(Output::new())
    }
}
