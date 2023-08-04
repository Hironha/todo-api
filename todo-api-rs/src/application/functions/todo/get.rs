use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct GetTodoPayload {
    pub id: String,
}

pub trait TodoGetter {
    fn get(&self, id: &str) -> Result<Todo, String>;
}

pub struct GetContext<T: TodoGetter> {
    pub store: T,
    pub payload: GetTodoPayload,
}

pub async fn get_todo<T: TodoGetter>(ctx: GetContext<T>) -> Result<Todo, String> {
    ctx.store.get(&ctx.payload.id)
}
