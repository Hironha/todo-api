use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
use crate::application::repositories::todo::{
    BindTagsError, ExistsError, ExistsTagsError, TodoRepository,
};

#[derive(Debug)]
pub struct BindTodoTagsUseCase<T> {
    repository: T,
}

impl<T: TodoRepository> BindTodoTagsUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn exec(&mut self, input: BindTodoTagsInput) -> Result<(), BindTodoTagsError> {
        match self.repository.exists(input.todo_id).await {
            Ok(exists) if !exists => return Err(BindTodoTagsError::TodoNotFound),
            Ok(_) => (),
            Err(err) => {
                return match err {
                    ExistsError::Internal(err) => Err(BindTodoTagsError::Internal(err)),
                };
            }
        };

        if let Err(err) = self.repository.exists_tags(&input.tags_id).await {
            return match err {
                ExistsTagsError::NotFound(tags_id) => Err(BindTodoTagsError::TagNotFound(tags_id)),
                ExistsTagsError::Internal(err) => Err(BindTodoTagsError::Internal(err)),
            };
        }

        self.repository
            .bind_tags(input.todo_id, &input.tags_id)
            .await
            .map_err(|err| match err {
                BindTagsError::Internal(err) => BindTodoTagsError::Internal(err),
            })
    }
}
