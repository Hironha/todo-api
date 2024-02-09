use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
use crate::application::repositories::todo::{
    BindTagsError, ExistsError, ExistsTagsError, TodoRepository,
};

#[derive(Debug)]
pub struct BindTodoTagsUseCase<T: TodoRepository> {
    todo_repository: T,
}

impl<T: TodoRepository> BindTodoTagsUseCase<T> {
    pub fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn exec(&mut self, input: BindTodoTagsInput) -> Result<(), BindTodoTagsError> {
        let todo_exists =
            self.todo_repository
                .exists(input.todo_id)
                .await
                .map_err(|err| match err {
                    ExistsError::Internal(err) => BindTodoTagsError::Internal(err),
                })?;

        if !todo_exists {
            return Err(BindTodoTagsError::TodoNotFound);
        }

        self.todo_repository
            .exists_tags(&input.tags_id)
            .await
            .map_err(|err| match err {
                ExistsTagsError::NotFound(tags_id) => BindTodoTagsError::TagNotFound(tags_id),
                ExistsTagsError::Internal(err) => BindTodoTagsError::Internal(err),
            })?;

        self.todo_repository
            .bind_tags(input.todo_id, &input.tags_id)
            .await
            .map_err(|err| match err {
                BindTagsError::Internal(err) => BindTodoTagsError::Internal(err),
            })
    }
}
