use crate::adapters::dtos::todo::list::{ListResponse, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::list::ListTodosInput;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::list::ListTodosUseCase;

#[derive(Clone, Debug)]
pub struct ListController<T>
where
    T: TodoRepository + Clone,
{
    todo_repository: T,
}

impl<T> ListController<T>
where
    T: TodoRepository + Clone,
{
    pub const fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<ListResponse, RunError>
    where
        R: Parse<ListTodosInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;

        let todo_list = ListTodosUseCase::new(self.todo_repository.clone())
            .exec(input)
            .await
            .map_err(RunError::Listing)?;

        let response = ListResponse {
            page: todo_list.page.into(),
            per_page: todo_list.per_page.into(),
            count: todo_list.count,
            items: todo_list
                .items
                .into_iter()
                .map(TodoPresenter::from)
                .collect(),
        };

        Ok(response)
    }
}
