use std::error::Error;

use crate::adapters::dtos::todo::create::ParseError;
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::create::CreateTodoInput;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::create::CreateTodoUseCase;

pub struct CreateController<T>
where
    T: TodoRepository + Clone,
{
    todo_repository: T,
}

impl<T: TodoRepository> CreateController<T>
where
    T: TodoRepository + Clone,
{
    pub const fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TodoPresenter, Box<dyn Error>>
    where
        R: Parse<CreateTodoInput, ParseError>,
    {
        let input = req.parse()?;

        CreateTodoUseCase::new(self.todo_repository.clone())
            .exec(input)
            .await
            .map_err(Box::from)
            .map(TodoPresenter::from)
    }
}
