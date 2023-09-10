use crate::adapters::dtos::todo::find::{Input, Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{find_todo, Find, FindContext, FindError, FindPayload};

pub struct FindController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Find,
{
    input: I,
    store: S,
}

impl<I, S> FindController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Find,
{
    pub const fn new(input: I, store: S) -> Self {
        Self { input, store }
    }

    pub async fn run(self) -> Output {
        let payload = match self.input.parse() {
            Ok(input) => FindPayload { id: input.id },
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = FindContext { store: self.store };
        let result = find_todo(ctx, payload).await.map_err(|err| match err {
            FindError::InternalError => RunError::Internal,
            FindError::NotFound => RunError::NotFound,
        });

        match result {
            Ok(todo) => Output::ok(TodoView::from(todo)),
            Err(err) => Output::err(err),
        }
    }
}
