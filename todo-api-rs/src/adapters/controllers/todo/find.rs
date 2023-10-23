use crate::adapters::dtos::todo::find::{Output, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::find::{FindTodoError, FindTodoInput};
use crate::application::functions::todo::{find_todo, FindTodoContext};
use crate::application::repositories::todo::find::Find;

#[derive(Clone, Debug)]
pub struct FindController<S: Find> {
    store: S,
}

impl<S: Find> FindController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(&self, input: impl Parse<FindTodoInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = FindTodoContext::new(&self.store);
        match find_todo(ctx, input).await.into_result() {
            Ok(todo) => Output::from_todo(todo),
            Err(err) => Output::err(match err {
                FindTodoError::Internal => RunError::Internal,
                FindTodoError::NotFound => RunError::NotFound,
            }),
        }
    }
}
