use std::error::Error;

use crate::adapters::dtos::todo::bind_tags::ParseError;
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::bind_tags::BindTodoTagsInput;
use crate::application::repositories::tag::TagRepository;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::bind_tags::BindTodoTagsUseCase;

#[derive(Clone, Debug)]
pub struct BindTagsController<T, S>
where
    T: TodoRepository + Clone,
    S: TagRepository + Clone,
{
    todo_repository: T,
    tag_repository: S,
}

impl<T, S> BindTagsController<T, S>
where
    T: TodoRepository + Clone,
    S: TagRepository + Clone,
{
    pub const fn new(todo_repository: T, tag_repository: S) -> Self {
        Self {
            todo_repository,
            tag_repository,
        }
    }

    pub async fn run<R>(&self, req: R) -> Result<(), Box<dyn Error>>
    where
        R: Parse<BindTodoTagsInput, ParseError>,
    {
        let input = req.parse()?;

        BindTodoTagsUseCase::new(self.todo_repository.clone(), self.tag_repository.clone())
            .exec(input)
            .await
            .map_err(Box::from)
    }
}
