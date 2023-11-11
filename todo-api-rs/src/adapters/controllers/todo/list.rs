use crate::adapters::dtos::todo::list::{ListResponse, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::dtos::todo::list::ListTodoInput;
use crate::application::functions::todo::list::{list_todo, ListTodoContext};
use crate::application::repositories::todo::list::List;

#[derive(Clone, Debug)]
pub struct ListController<Repo: List> {
    repository: Repo,
}

impl<Repo: List> ListController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run<Req>(&self, req: Req) -> Result<ListResponse, RunError>
    where
        Req: Parse<ListTodoInput, ParseError>,
    {
        let input = req.parse().map_err(RunError::Parsing)?;
        let context = ListTodoContext::new(&self.repository);
        let todo_list = list_todo(context, input).await.map_err(RunError::Listing)?;
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
