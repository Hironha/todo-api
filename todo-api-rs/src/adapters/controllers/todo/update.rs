use crate::adapters::dtos::todo::update::{Output, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput};
use crate::application::functions::todo::update::{update_todo, UpdateTodoContext};
use crate::application::repositories::todo::update::Update;

pub struct UpdateController<S: Update> {
    store: S,
}

impl<S: Update> UpdateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl Parse<UpdateTodoInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = UpdateTodoContext { store: self.store };
        let result = update_todo(ctx, input).await.into_result();

        match result {
            Ok(todo) => Output::from_todo(todo),
            Err(err) => Output::err(match err {
                UpdateTodoError::NotFound => RunError::NotFound,
                UpdateTodoError::Internal => RunError::Internal,
            }),
        }
    }
}
