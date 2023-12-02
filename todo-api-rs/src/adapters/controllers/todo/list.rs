use crate::adapters::dtos::todo::list::{ListResponse, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::list::ListTodoInput;
use crate::application::functions::todo::list::{list_todo, ListTodoContext};
use crate::application::repositories::todo::TodoRepository;

#[derive(Clone, Debug)]
pub struct ListController<T>
where
    T: TodoRepository,
{
    repository: T,
}

impl<T> ListController<T>
where
    T: TodoRepository,
{
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn run<R>(&self, req: R) -> Result<ListResponse, RunError>
    where
        R: Parse<ListTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let ctx = ListTodoContext {
            todo_repository: &self.repository,
        };

        let todo_list = list_todo(ctx, input).await.map_err(RunError::Listing)?;

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
