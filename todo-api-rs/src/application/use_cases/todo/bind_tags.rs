use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
use crate::application::repositories::tag::{ExistsManyError, TagRepository};
use crate::application::repositories::todo::{BindTagsError, ExistsError, TodoRepository};

#[derive(Debug)]
pub struct BindTodoTagsUseCase<T: TodoRepository, S: TagRepository> {
    todo_repository: T,
    tag_repository: S,
}

impl<T: TodoRepository, S: TagRepository> BindTodoTagsUseCase<T, S> {
    pub fn new(todo_repository: T, tag_repository: S) -> Self {
        Self {
            todo_repository,
            tag_repository,
        }
    }

    pub async fn exec(&self, input: BindTodoTagsInput) -> Result<(), BindTodoTagsError> {
        let todo_exists =
            self.todo_repository
                .exists(input.todo_id)
                .await
                .map_err(|err| match err {
                    ExistsError::Internal(err) => BindTodoTagsError::Repository(err),
                })?;

        if !todo_exists {
            return Err(BindTodoTagsError::TodoNotFound);
        }

        self.tag_repository
            .exists_many(&input.tags_id)
            .await
            .map_err(|err| match err {
                ExistsManyError::NotFound(tags_id) => BindTodoTagsError::TagNotFound(tags_id),
                ExistsManyError::Internal(err) => BindTodoTagsError::Repository(err),
            })?;

        self.todo_repository
            .bind_tags(input.todo_id, input.tags_id)
            .await
            .map_err(|err| match err {
                BindTagsError::Internal(err) => BindTodoTagsError::Repository(err),
            })
    }
}
