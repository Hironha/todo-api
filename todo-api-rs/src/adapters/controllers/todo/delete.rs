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
        let context = DeleteContext { store: self.store };
        delete_todo(&context, input.into_payload())
            .await
            .map(|_| Output::new())
            .map_err(|e| e.run_error())
    }
}

impl Input {
    fn into_payload(self) -> DeletePayload {
        DeletePayload { id: self.id }
    }
}

impl DeleteError {
    fn run_error(&self) -> RunError {
        match self {
            Self::InternalError => RunError::Internal,
            Self::NotFound => RunError::NotFound,
        }
    }
}
