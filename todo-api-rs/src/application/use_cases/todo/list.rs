use crate::application::dtos::todo::list::{ListTodoError, ListTodosInput, TodosList};
use crate::application::repositories::todo::{ListError, ListQuery, TodoRepository};

#[derive(Debug)]
pub struct ListTodosUseCase<T> {
    todo_repository: T,
}

impl<T: TodoRepository> ListTodosUseCase<T> {
    pub fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn exec(&mut self, input: ListTodosInput) -> Result<TodosList, ListTodoError> {
        let payload = ListQuery {
            page: input.page,
            per_page: input.per_page,
            title: input.title,
        };

        let list_data = self
            .todo_repository
            .list(payload)
            .await
            .map_err(|err| match err {
                ListError::Internal(err) => ListTodoError::Internal(err),
            })?;

        Ok(TodosList {
            count: list_data.count,
            items: list_data.items,
            page: input.page,
            per_page: input.per_page,
        })
    }
}
