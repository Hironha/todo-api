use crate::adapters::dtos::todo::find::{Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{
    find_todo, FindTodoContext, FindTodoError, FindTodoInput,
};
use crate::application::repositories::todo::find::Find;

pub struct FindController<S: Find> {
    store: S,
}

impl<S: Find> FindController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<FindTodoInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = FindTodoContext { store: self.store };
        let result = find_todo(ctx, input).await.map_err(|err| match err {
            FindTodoError::Internal => RunError::Internal,
            FindTodoError::NotFound => RunError::NotFound,
        });

        match result {
            Ok(todo) => Output::ok(TodoView::from(todo)),
            Err(err) => Output::err(err),
        }
    }
}
