use crate::adapters::dtos::todo::find::{ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::find::FindTodoUseCase;
use crate::domain::types::Id;

#[derive(Clone, Debug)]
pub struct FindController<T>
where
    T: TodoRepository + Clone,
{
    todo_repository: T,
}

impl<T> FindController<T>
where
    T: TodoRepository + Clone,
{
    pub const fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<TodoPresenter, RunError>
    where
        R: Parse<Id, ParseError>,
    {
        let todo_id = req.parse().map_err(RunError::Parsing)?;

        FindTodoUseCase::new(self.todo_repository.clone())
            .exec(todo_id)
            .await
            .map(TodoPresenter::from)
            .map_err(RunError::Finding)
    }
}
