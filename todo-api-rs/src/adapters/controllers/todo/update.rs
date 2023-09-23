use crate::adapters::dtos::todo::update::{Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{
    update_todo, UpdateTodoContext, UpdateTodoError, UpdateTodoInput,
};
use crate::application::repositories::todo::update::Update;

pub struct UpdateController<S: Update> {
    store: S,
}

impl<S: Update> UpdateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<UpdateTodoInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = UpdateTodoContext { store: self.store };
        let result = update_todo(ctx, input).await.map_err(|err| match err {
            UpdateTodoError::NotFound => RunError::NotFound,
            UpdateTodoError::Internal => RunError::Internal,
        });

        match result {
            Ok(todo) => Output::ok(TodoView::from(todo)),
            Err(err) => Output::err(err),
        }
    }
}
