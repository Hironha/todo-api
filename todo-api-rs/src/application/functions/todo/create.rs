use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput};
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::domain::entities::todo::TodoEntity;

pub async fn create_todo<Repo: Create>(
    ctx: CreateTodoContext<'_, Repo>,
    input: CreateTodoInput,
) -> Result<TodoEntity, CreateTodoError> {
    let payload = CreatePayload {
        title: input.title,
        description: input.description,
        todo_at: input.todo_at,
        status: input.status,
    };

    ctx.repository
        .create(payload)
        .await
        .map_err(|err| match err {
            CreateError::Internal(err) => CreateTodoError::Repository(err),
        })
}

#[derive(Clone, Debug)]
pub struct CreateTodoContext<'a, Repo: Create> {
    repository: &'a Repo,
}

impl<'a, Repo: Create> CreateTodoContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
