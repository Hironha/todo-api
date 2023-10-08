use crate::application::dtos::todo::list::{ListTodoError, ListTodoInput, ListTodoOutput};
use crate::application::repositories::todo::list::{List, ListError, ListPayload};

pub async fn list_todo<S: List>(
    ctx: ListTodoContext<'_, S>,
    input: ListTodoInput,
) -> ListTodoOutput {
    let payload = ListPayload {
        page: input.page,
        per_page: input.per_page,
    };

    match ctx.store.list(payload).await {
        Ok(todos) => ListTodoOutput::ok(todos),
        Err(err) => ListTodoOutput::err(match err {
            ListError::Internal => ListTodoError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct ListTodoContext<'a, S: List> {
    store: &'a S,
}

impl<'a, S: List> ListTodoContext<'a, S> {
    pub const fn new(store: &'a S) -> Self {
        Self { store }
    }
}
