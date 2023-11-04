use crate::adapters::dtos::todo::bind_tags::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::bind_tags::{BindTodoTagsError, BindTodoTagsInput};
use crate::application::functions::todo::bind_tags::{bind_todo_tags, BindTodoTagsContext};
use crate::application::repositories::todo::bind_tags::BindTags;

#[derive(Clone, Debug)]
pub struct BindTagsController<Repo: BindTags> {
    repository: Repo,
}

impl<Repo: BindTags> BindTagsController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<(), RunError>
    where
        Req: Parse<BindTodoTagsInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = BindTodoTagsContext::new(&self.repository);

        bind_todo_tags(ctx, input).await.map_err(|err| match err {
            BindTodoTagsError::Internal => RunError::Internal,
            BindTodoTagsError::TodoNotFound => RunError::TodoNotFound,
            BindTodoTagsError::TagNotFound => RunError::TagNotFound,
        })
    }
}
