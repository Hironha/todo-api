use crate::adapters::dtos::tag::update::{Output, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::update::{UpdateTagError, UpdateTagInput};
use crate::application::functions::tag::update::{update_tag, UpdateTagContext};
use crate::application::repositories::tag::update::Update;

#[derive(Clone, Debug)]
pub struct UpdateController<S: Update> {
    store: S,
}

impl<S: Update> UpdateController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(&self, input: impl Parse<UpdateTagInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let ctx = UpdateTagContext::new(&self.store);
        match update_tag(ctx, input).await.into_result() {
            Ok(tag) => Output::from_tag(tag),
            Err(err) => Output::err(match err {
                UpdateTagError::NotFound => RunError::NotFound,
                UpdateTagError::Internal => RunError::Internal,
            }),
        }
    }
}
