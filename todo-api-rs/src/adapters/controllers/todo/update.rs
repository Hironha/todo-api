use std::error::Error;

use crate::adapters::dtos::todo::update::ParseError;
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::update::UpdateTodoInput;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::update::UpdateTodoUseCase;

pub struct UpdateController<T>
where
    T: TodoRepository + Clone,
{
    todo_repository: T,
}

impl<T> UpdateController<T>
where
    T: TodoRepository + Clone,
{
    pub const fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn run<R>(self, req: R) -> Result<(), Box<dyn Error>>
    where
        R: Parse<UpdateTodoInput, ParseError>,
    {
        let input = req.parse()?;

        UpdateTodoUseCase::new(self.todo_repository.clone())
            .exec(input)
            .await
            .map_err(Box::from)
    }
}
