use crate::adapters::dtos::tag::find::{Output, ParseError, RunError};
use crate::adapters::dtos::ParsableInput;
use crate::application::dtos::tag::find::{FindTagError, FindTagInput};
use crate::application::functions::tag::find::{find_tag, FindTagContext};
use crate::application::repositories::tag::find::Find;

#[derive(Clone, Debug)]
pub struct FindController<S: Find> {
    store: S,
}

impl<S: Find> FindController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self, input: impl ParsableInput<FindTagInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let ctx = FindTagContext { store: self.store };
        match find_tag(ctx, input).await.into_result() {
            Ok(tag) => Output::from_tag(tag),
            Err(err) => Output::err(match err {
                FindTagError::NotFound => RunError::NotFound,
                FindTagError::Internal => RunError::Internal,
            }),
        }
    }
}
