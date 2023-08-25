use async_trait::async_trait;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub enum DeleteError {
    NotFound,
    InternalError,
}

impl std::fmt::Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "NotFound"),
            Self::InternalError => writeln!(f, "InternalError"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeletePayload {
    pub id: Id,
}

#[async_trait]
pub trait Delete {
    async fn delete(&self, id: &Id) -> Result<(), DeleteError>;
}

pub struct DeleteContext<T: Delete> {
    pub store: T,
}

pub async fn delete_todo<T: Delete>(
    ctx: &DeleteContext<T>,
    payload: &DeletePayload,
) -> Result<(), DeleteError> {
    ctx.store.delete(&payload.id).await
}
