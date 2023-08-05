use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct GetPayload {
    pub id: String,
}

pub trait TodoGetter {
    fn get(&self, id: &str) -> Result<Todo, String>;
}

pub struct GetContext<T: TodoGetter> {
    pub store: T,
}

pub async fn get_todo<T: TodoGetter>(
    ctx: GetContext<T>,
    payload: &GetPayload,
) -> Result<Todo, String> {
    ctx.store.get(&payload.id)
}
