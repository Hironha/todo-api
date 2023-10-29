use crate::application::dtos::todo::find::{FindTodoError, FindTodoInput};
use crate::application::repositories::todo::find::{Find, FindError};
use crate::domain::entities::todo::TodoEntity;

pub async fn find_todo<Repo: Find>(
    ctx: FindTodoContext<'_, Repo>,
    FindTodoInput(id): FindTodoInput,
) -> Result<TodoEntity, FindTodoError> {
    ctx.repository.find(id).await.map_err(|err| match err {
        FindError::NotFound => FindTodoError::NotFound,
        FindError::Internal => FindTodoError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct FindTodoContext<'a, Repo: Find> {
    repository: &'a Repo,
}

impl<'a, Repo: Find> FindTodoContext<'a, Repo> {
    pub const fn new(repository: &'a Repo) -> Self {
        Self { repository }
    }
}
