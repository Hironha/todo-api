use crate::application::dtos::todo::list::{ListTodoError, ListTodoInput, TodoList};
use crate::application::repositories::todo::{ListError, ListPayload, TodoRepository};

pub async fn list_todo<T>(
    ctx: ListTodoContext<'_, T>,
    input: ListTodoInput,
) -> Result<TodoList, ListTodoError>
where
    T: TodoRepository,
{
    let payload = ListPayload {
        page: input.page,
        per_page: input.per_page,
        title: input.title,
    };

    let list_data = ctx
        .todo_repository
        .list(payload)
        .await
        .map_err(|err| match err {
            ListError::Internal(err) => ListTodoError::Repository(err),
        })?;

    Ok(TodoList {
        count: list_data.count,
        items: list_data.items,
        page: input.page,
        per_page: input.per_page,
    })
}

#[derive(Clone, Debug)]
pub struct ListTodoContext<'a, T>
where
    T: TodoRepository,
{
    pub todo_repository: &'a T,
}
