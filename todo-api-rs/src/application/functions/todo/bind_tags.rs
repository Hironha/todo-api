use crate::application::dtos::todo::bind_tags::{
    BindTodoTagsError, BindTodoTagsInput, BindTodoTagsOutput,
};
use crate::application::repositories::todo::bind_tags::{BindTags, BindTagsError, BindTagsPayload};

pub async fn bind_todo_tags<R: BindTags>(
    ctx: BindTodoTagsContext<'_, R>,
    input: BindTodoTagsInput,
) -> BindTodoTagsOutput {
    let payload = BindTagsPayload {
        tags_id: input.tags_id,
        todo_id: input.todo_id,
    };

    match ctx.repository.bind_tags(payload).await {
        Ok(_) => BindTodoTagsOutput::ok(),
        Err(err) => BindTodoTagsOutput::err(match err {
            BindTagsError::TagNotFound => BindTodoTagsError::TagNotFound,
            BindTagsError::TodoNotFound => BindTodoTagsError::TodoNotFound,
            BindTagsError::Internal => BindTodoTagsError::Internal,
        }),
    }
}

#[derive(Clone, Debug)]
pub struct BindTodoTagsContext<'r, R: BindTags> {
    repository: &'r R,
}

impl<'r, R: BindTags> BindTodoTagsContext<'r, R> {
    pub const fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}
