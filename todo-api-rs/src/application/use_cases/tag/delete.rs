use crate::application::dtos::tag::delete::{DeleteTagError, DeleteTagInput, DeleteTagOutput};
use crate::application::repositories::tag::{DeleteError, TagRepository};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct DeleteTagUseCase<T> {
    repository: T,
}

impl<T: TagRepository> DeleteTagUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T: TagRepository> UseCase<DeleteTagInput, DeleteTagOutput> for DeleteTagUseCase<T> {
    async fn exec(mut self, input: DeleteTagInput) -> DeleteTagOutput {
        self.repository
            .delete(input)
            .await
            .map_err(|err| match err {
                DeleteError::NotFound => DeleteTagError::NotFound,
                DeleteError::Internal(err) => DeleteTagError::Internal(err),
            })
    }
}
