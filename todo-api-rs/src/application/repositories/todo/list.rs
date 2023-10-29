use async_trait::async_trait;
use std::num::NonZeroU32;

use crate::domain::entities::todo::{TodoEntity, Title};

#[async_trait]
pub trait List {
    async fn list(&self, payload: ListPayload) -> Result<ListData, ListError>;
}

#[derive(Clone, Debug)]
pub struct ListPayload {
    pub page: NonZeroU32,
    pub per_page: NonZeroU32,
    pub title: Option<Title> 
}

#[derive(Clone, Debug)]
pub struct ListData {
    pub count: u64,
    pub items: Vec<TodoEntity>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListError {
    Internal,
}
