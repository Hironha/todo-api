use async_trait::async_trait;

use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct DeletePayload {
    pub id: Id,
}

#[async_trait]
pub trait Delete {
    async fn delete(&self, id: &Id) -> Result<(), String>;
}

pub struct DeleteContext<T: Delete> {
    pub store: T,
}

pub async fn delete_todo<T: Delete>(
    ctx: &DeleteContext<T>,
    payload: &DeletePayload,
) -> Result<(), String> {
    ctx.store.delete(&payload.id).await
}
