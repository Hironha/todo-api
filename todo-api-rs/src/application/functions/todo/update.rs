use crate::domain::todo::Todo;

#[derive(Clone, Debug)]
pub struct UpdatePayload {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>
}

pub trait TodoSetter {
    fn set(&self, payload: UpdatePayload) -> Result<Todo, String>;
}

pub struct UpdateContext<T: TodoSetter> {
    pub store: T,
}

pub async fn update_todo<T: TodoSetter>(
    ctx: &UpdateContext<T>,
    payload: UpdatePayload,
) -> Result<Todo, String> {
    ctx.store.set(payload)
}
