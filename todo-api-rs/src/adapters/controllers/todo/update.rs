use crate::adapters::dtos::todo::update::{Input, Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{
    update_todo, Update, UpdateContext, UpdateError, UpdatePayload,
};

pub struct UpdateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Update,
{
    input: I,
    store: S,
}

impl<I, S> UpdateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Update,
{
    pub const fn new(input: I, store: S) -> Self {
        Self { input, store }
    }

    pub async fn run(self) -> Output {
        let payload = match self.input.parse() {
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
