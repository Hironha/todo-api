use crate::adapters::dtos::todo::update::{Input, Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{
    update_todo, Update, UpdateContext, UpdateError, UpdatePayload,
};

pub struct UpdateController<S: Update> {
    store: S,
}

impl<S: Update> UpdateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<Input, ParseError>) -> Output {
        let payload = match input.parse() {
            Ok(input) => UpdatePayload {
                id: input.id,
                title: input.title,
                description: input.description,
                todo_at: input.todo_at,
            },
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = UpdateContext { store: self.store };
        let result = update_todo(ctx, payload).await.map_err(|err| match err {
            UpdateError::NotFound => RunError::NotFound,
            UpdateError::InternalError => RunError::Internal,
        });

        match result {
            Ok(todo) => Output::ok(TodoView::from(todo)),
            Err(err) => Output::err(err),
        }
    }
}
