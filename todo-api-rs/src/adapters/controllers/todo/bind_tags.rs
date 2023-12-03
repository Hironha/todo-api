use crate::adapters::dtos::todo::bind_tags::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::bind_tags::BindTodoTagsInput;
use crate::application::functions::todo::bind_tags::{bind_todo_tags, BindTodoTagsContext};
use crate::application::repositories::tag::TagRepository;
use crate::application::repositories::todo::TodoRepository;

#[derive(Clone, Debug)]
pub struct BindTagsController<T, S> {
    todo_repository: T,
    tag_repository: S,
}

impl<T, S> BindTagsController<T, S>
where
    T: TodoRepository,
    S: TagRepository,
{
    pub const fn new(todo_repository: T, tag_repository: S) -> Self {
        Self {
            todo_repository,
            tag_repository,
        }
    }

    pub async fn run<R>(&self, req: R) -> Result<(), RunError>
    where
        R: Parse<BindTodoTagsInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = BindTodoTagsContext {
            tag_repository: &self.tag_repository,
            todo_repository: &self.todo_repository,
        };

        bind_todo_tags(ctx, input).await.map_err(RunError::Binding)
    }
}
