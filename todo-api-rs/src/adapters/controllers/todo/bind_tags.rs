use crate::adapters::dtos::todo::bind_tags::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::bind_tags::BindTodoTagsInput;
use crate::application::functions::todo::bind_tags::{bind_todo_tags, BindTodoTagsContext};
use crate::application::repositories::tag::exists_all::ExistsAll;
use crate::application::repositories::todo::bind_tags::BindTags;
use crate::application::repositories::todo::exists::Exists;

#[derive(Clone, Debug)]
pub struct BindTagsController<TodoRepo, TagRepo>
where
    TodoRepo: BindTags + Exists,
    TagRepo: ExistsAll,
{
    todo_repository: TodoRepo,
    tag_repository: TagRepo,
}

impl<TodoRepo, TagRepo> BindTagsController<TodoRepo, TagRepo>
where
    TodoRepo: BindTags + Exists,
    TagRepo: ExistsAll,
{
    pub const fn new(todo_repository: TodoRepo, tag_repository: TagRepo) -> Self {
        Self {
            todo_repository,
            tag_repository,
        }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<(), RunError>
    where
        Req: Parse<BindTodoTagsInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = BindTodoTagsContext::new(&self.todo_repository, &self.tag_repository);
        bind_todo_tags(ctx, input).await.map_err(RunError::Binding)
    }
}
