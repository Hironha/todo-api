use crate::application::dtos::tag::find::{FindTagError, FindTagInput, FindTagOutput};
use crate::application::repositories::tag::{FindError, TagRepository};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct FindTagUseCase<T> {
    repository: T,
}

impl<T: TagRepository> FindTagUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TagRepository> UseCase<FindTagInput, FindTagOutput> for FindTagUseCase<T> {
    async fn exec(self, input: FindTagInput) -> FindTagOutput {
        self.repository.find(input).await.map_err(|err| match err {
            FindError::NotFound => FindTagError::NotFound,
            FindError::Internal(err) => FindTagError::Internal(err),
        })
    }
}
