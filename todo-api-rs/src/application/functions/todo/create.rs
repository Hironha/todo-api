use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct CreatePayload {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

pub trait TodoCreator {
    fn create(&mut self, payload: CreatePayload) -> Result<Todo, String>;
}

pub struct CreateContext<T: TodoCreator> {
    pub store: T,
}

pub async fn create_todo<T: TodoCreator>(
    ctx: &mut CreateContext<T>,
    payload: CreatePayload,
) -> Result<Todo, String> {
    ctx.store.create(payload)
}
