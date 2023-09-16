use crate::adapters::dtos::todo::create::{Input, Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{
    create_todo, Create, CreateContext, CreateError, CreatePayload,
};

pub struct CreateController<S: Create> {
    store: S,
}

impl<S: Create> CreateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<Input, ParseError>) -> Output {
        let payload = match input.parse() {
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
