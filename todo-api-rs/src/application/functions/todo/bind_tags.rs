use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
use crate::application::repositories::tag::exists_all::{ExistsAll, ExistsAllError};
use crate::application::repositories::todo::{BindTagsError, ExistsError, TodoRepository};

pub async fn bind_todo_tags<T, S>(
    ctx: BindTodoTagsContext<'_, T, S>,
    input: BindTodoTagsInput,
) -> Result<(), BindTodoTagsError>
where
    T: TodoRepository,
    S: ExistsAll,
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

    ctx.todo_repository
        .bind_tags(input.todo_id, input.tags_id)
        .await
        .map_err(|err| match err {
            BindTagsError::Internal(err) => BindTodoTagsError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct BindTodoTagsContext<'a, T, S>
where
    T: TodoRepository,
    S: ExistsAll,
{
    pub todo_repository: &'a T,
    pub tag_repository: &'a S,
}
