use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct DeletePayload {
    pub id: String,
}

pub trait TodoDeleter {
    fn delete(&mut self, id: &str) -> Result<Todo, String>;
}

pub struct DeleteContext<T: TodoDeleter> {
    pub store: T,
}

pub async fn delete_todo<T: TodoDeleter>(
    ctx: &mut DeleteContext<T>,
    payload: &DeletePayload,
) -> Result<Todo, String> {
    ctx.store.delete(&payload.id)
}
