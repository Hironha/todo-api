use crate::adapters::dtos::tag::delete::{Output, ParseError, RunError};
use crate::adapters::dtos::Parse;
use crate::application::dtos::tag::delete::{DeleteTagError, DeleteTagInput};
use crate::application::functions::tag::delete::{delete_tag, DeleteTagContext};
use crate::application::repositories::tag::delete::Delete;

#[derive(Clone, Debug)]
pub struct DeleteController<S: Delete> {
    store: S,
}

impl<S: Delete> DeleteController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(&self, input: impl Parse<DeleteTagInput, ParseError>) -> Output {
        let input = match input.parse() {
            Ok(input) => input,
            Err(err) => return Output::err(RunError::Parsing(err)),
        };

        let ctx =  DeleteTagContext::new(&self.store);
        match delete_tag(ctx, input).await.into_result() {
            Ok(_) => Output::ok(),
            Err(err) => Output::err(match err {
                DeleteTagError::NotFound => RunError::NotFound,
                DeleteTagError::Internal => RunError::Internal,
            }),
        }
    }
}
