use crate::adapters::dtos::tag::create::{Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::application::dto::tag::create::{CreateTagInput, CreateTagError};
use crate::application::functions::tag::create::{create_tag, CreateTagContext};
use crate::application::repositories::tag::create::Create;

pub struct CreateController<S: Create> {
    store: S,
}

impl<S: Create> CreateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<CreateTagInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let ctx = CreateTagContext { store: self.store };
        let result = create_tag(ctx, input).await.into_result();

        match result {
            Ok(tag) => Output::from_tag(tag),
            Err(err) => Output::err(match err {
                CreateTagError::Internal => RunError::Internal,
            }),
        }
    }
}
