use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
use crate::application::repositories::tag::exists_all::{ExistsAll, ExistsAllError};
use crate::application::repositories::todo::bind_tags::{BindTags, BindTagsError, BindTagsPayload};
use crate::application::repositories::todo::exists::{Exists, ExistsError};

pub async fn bind_todo_tags<TodoRepo, TagRepo>(
    ctx: BindTodoTagsContext<'_, TodoRepo, TagRepo>,
    input: BindTodoTagsInput,
) -> Result<(), BindTodoTagsError>
where
    TodoRepo: BindTags + Exists,
    TagRepo: ExistsAll,
{
    let todo_exists = ctx
        .todo_repository
        .exists(input.todo_id)
        .await
        .map_err(|err| match err {
            ExistsError::Internal(err) => BindTodoTagsError::Repository(err),
        })?;

    if !todo_exists {
        return Err(BindTodoTagsError::TodoNotFound);
    }

    ctx.tag_repository
        .exists_all(&input.tags_id)
        .await
        .map_err(|err| match err {
            ExistsAllError::NotFound(tags_id) => BindTodoTagsError::TagNotFound(tags_id),
            ExistsAllError::Internal(err) => BindTodoTagsError::Repository(err),
        })?;

    let bind_tags_payload = BindTagsPayload {
        tags_id: input.tags_id,
        todo_id: input.todo_id,
    };

    ctx.todo_repository
        .bind_tags(bind_tags_payload)
        .await
        .map_err(|err| match err {
            BindTagsError::Internal(err) => BindTodoTagsError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct BindTodoTagsContext<'a, TodoRepo, TagRepo>
where
    TodoRepo: BindTags + Exists,
    TagRepo: ExistsAll,
{
    todo_repository: &'a TodoRepo,
    tag_repository: &'a TagRepo,
}

impl<'a, TodoRepo, TagRepo> BindTodoTagsContext<'a, TodoRepo, TagRepo>
where
    TodoRepo: BindTags + Exists,
    TagRepo: ExistsAll,
{
    pub const fn new(todo_repository: &'a TodoRepo, tag_repository: &'a TagRepo) -> Self {
        Self {
            todo_repository,
            tag_repository,
        }
    }
}
