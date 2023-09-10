use crate::adapters::dtos::todo::create::{Input, Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{
    create_todo, Create, CreateContext, CreateError, CreatePayload,
};

pub struct CreateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Create,
{
    store: S,
    input: I,
}

impl<I, S> CreateController<I, S>
where
    I: ParsableInput<Input, ParseError>,
    S: Create,
{
    pub const fn new(input: I, store: S) -> Self {
        Self { input, store }
    }

    pub async fn run(self) -> Output {
        let payload = match self.input.parse() {
            Ok(input) => CreatePayload {
                title: input.title,
                description: input.description,
                todo_at: input.todo_at,
            },
            Err(err) => return Output::err(RunError::Validation(err)),
        };

        let ctx = CreateContext { store: self.store };
        let result = create_todo(ctx, payload).await.map_err(|err| match err {
            CreateError::Internal => RunError::Internal,
        });

        match result {
            Ok(todo) => Output::ok(TodoView::from(todo)),
            Err(err) => Output::err(err),
        }
    }
}
