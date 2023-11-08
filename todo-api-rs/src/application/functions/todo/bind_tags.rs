use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
use crate::application::repositories::todo::bind_tags::{BindTags, BindTagsError, BindTagsPayload};
use crate::domain::types::DateTime;

pub async fn bind_todo_tags<Repo: BindTags>(
    ctx: BindTodoTagsContext<'_, Repo>,
    input: BindTodoTagsInput,
) -> Result<(), BindTodoTagsError> {
    let payload = BindTagsPayload {
        tags_id: input.tags_id,
        todo_id: input.todo_id,
        current_dt: DateTime::new(),
    };

    ctx.repository
        .bind_tags(payload)
        .await
        .map_err(|err| match err {
            BindTagsError::TagNotFound => BindTodoTagsError::TagNotFound,
            BindTagsError::TodoNotFound => BindTodoTagsError::TodoNotFound,
            BindTagsError::Internal(err) => BindTodoTagsError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct BindTodoTagsContext<'a, Repo: BindTags> {
    repository: &'a Repo,
}

impl<'a, Repo: BindTags> BindTodoTagsContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
