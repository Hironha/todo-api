use async_trait::async_trait;
use std::num::NonZeroU32;

use crate::domain::entities::todo::TodoEntity;

#[async_trait]
pub trait List {
    async fn list(&self, payload: ListPayload) -> Result<Vec<TodoEntity>, ListError>;
}

#[derive(Clone, Debug)]
pub struct ListPayload{
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListError {
    Internal,
}
