use crate::adapters::dtos::todo::list::{Output, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::list::{ListTodoError, ListTodoInput};
use crate::application::functions::todo::list::{list_todo, ListTodoContext};
use crate::application::repositories::todo::list::List;

#[derive(Clone, Debug)]
pub struct ListController<S: List> {
    store: S,
}

impl<S: List> ListController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(&self, input: impl Parse<ListTodoInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let context = ListTodoContext::new(&self.store);
        match list_todo(context, input).await.into_result() {
            Ok(list) => Output::from_list(list),
            Err(err) => Output::err(match err {
                ListTodoError::Internal => RunError::Internal,
            }),
        }
    }
}
