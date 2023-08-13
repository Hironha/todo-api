use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DeletePayload {
    pub id: Uuid,
}

#[async_trait]
pub trait TodoDeleter {
    async fn delete(&self, id: &Uuid) -> Result<(), String>;
}

pub struct DeleteContext<T: TodoDeleter> {
    pub store: T,
}

pub async fn delete_todo<T: TodoDeleter>(
    ctx: &DeleteContext<T>,
    payload: &DeletePayload,
) -> Result<(), String> {
    ctx.store.delete(&payload.id).await
}
