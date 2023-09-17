use crate::adapters::dtos::todo::create::{Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{
    create_todo, CreateContext, CreateTodoError, CreateTodoInput,
};
use crate::application::repositories::todo::create::Create;

pub struct CreateController<S: Create> {
    store: S,
}

impl<S: Create> CreateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<CreateTodoInput, ParseError>) -> Output {
        let create_input = match input.parse() {
            Ok(i) => i,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let ctx = CreateContext { store: self.store };
        let result = create_todo(ctx, create_input).await.map_err(|e| match e {
            CreateTodoError::Title(e) => RunError::InvalidTitle(e.description()),
            CreateTodoError::Description(e) => RunError::InvalidDescription(e.description()),
            CreateTodoError::Internal => RunError::Internal,
        });

        match result {
            Ok(todo) => Output::ok(TodoView::from(todo)),
            Err(err) => Output::err(err),
        }
    }
}
