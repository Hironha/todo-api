use crate::application::dtos::tag::delete::DeleteTagError;
use crate::application::repositories::tag::{DeleteError, TagRepository};
use crate::domain::types::Id;

#[derive(Debug)]
pub struct DeleteTagUseCase<T: TagRepository> {
    tag_repository: T,
}

impl<T: TagRepository> DeleteTagUseCase<T> {
    pub fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn exec(&self, tag_id: Id) -> Result<(), DeleteTagError> {
        self.tag_repository
            .delete(tag_id)
            .await
            .map_err(|err| match err {
                DeleteError::NotFound => DeleteTagError::NotFound,
                DeleteError::Internal(err) => DeleteTagError::Repository(err),
            })
    }
}
