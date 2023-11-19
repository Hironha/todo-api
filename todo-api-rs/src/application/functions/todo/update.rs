use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::application::repositories::todo::update::{Update, UpdateError, UpdatePayload};
use crate::domain::types::DateTime;

pub async fn update_todo<Repo: Update>(
    ctx: UpdateTodoContext<'_, Repo>,
    input: UpdateTodoInput,
) -> Result<(), UpdateTodoError> {
    let payload = UpdatePayload {
        id: input.id,
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
        status: input.status,
        updated_at: DateTime::new(),
    };

    ctx.repository
        .update(payload)
        .await
        .map_err(|err| match err {
            UpdateError::NotFound => UpdateTodoError::NotFound,
            UpdateError::Internal(err) => UpdateTodoError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct UpdateTodoContext<'a, Repo: Update> {
    repository: &'a Repo,
}

impl<'a, Repo: Update> UpdateTodoContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
