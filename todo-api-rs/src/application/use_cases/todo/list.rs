use crate::application::dtos::todo::list::{
    ListTodosError, ListTodosInput, ListTodosOutput, TodosList,
};
use crate::application::repositories::todo::{ListError, ListQuery, TodoRepository};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct ListTodosUseCase<T> {
    repository: T,
}

impl<T: TodoRepository> ListTodosUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TodoRepository> UseCase<ListTodosInput, ListTodosOutput> for ListTodosUseCase<T> {
    async fn exec(self, input: ListTodosInput) -> ListTodosOutput {
        let payload = ListQuery {
            page: input.page,
            per_page: input.per_page,
            title: input.title,
        };

        let list_data = self
            .repository
            .list(payload)
            .await
            .map_err(|err| match err {
                ListError::Internal(err) => ListTodosError::Internal(err),
            })?;

        Ok(TodosList {
            count: list_data.count,
            items: list_data.items,
            page: input.page,
            per_page: input.per_page,
        })
    }
}
