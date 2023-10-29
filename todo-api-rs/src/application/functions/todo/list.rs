use crate::application::dtos::todo::list::{ListTodoError, ListTodoInput, TodoList};
use crate::application::repositories::todo::list::{List, ListError, ListPayload};

pub async fn list_todo<Repo: List>(
    ctx: ListTodoContext<'_, Repo>,
    input: ListTodoInput,
) -> Result<TodoList, ListTodoError> {
    let payload = ListPayload {
        page: input.page,
        per_page: input.per_page,
        title: input.title,
    };

    let list_data = ctx
        .repository
        .list(payload)
        .await
        .map_err(|err| match err {
            ListError::Internal => ListTodoError::Internal,
        })?;

    Ok(TodoList {
        count: list_data.count,
        items: list_data.items,
        page: input.page,
        per_page: input.per_page,
    })
}

#[derive(Clone, Debug)]
pub struct ListTodoContext<'a, Repo: List> {
    repository: &'a Repo,
}

impl<'a, Repo: List> ListTodoContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
