use std::error::Error;

use crate::adapters::dtos::todo::list::{ListResponse, ParseError};
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

    pub async fn run<R>(&self, req: R) -> Result<ListResponse, Box<dyn Error>>
    where
        R: Parse<ListTodosInput, ParseError>,
    {
        let input = req.parse()?;

        let todo_list = ListTodosUseCase::new(self.todo_repository.clone())
            .exec(input)
            .await?;

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
