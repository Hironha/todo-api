use std::error::Error;

use crate::application::dtos::tag::list_all::ListAllTagsOutput;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::list_all::ListAllTagsUseCase;

pub trait ListAllTagsPresenter {
    type View;
    fn present(&self, result: Result<ListAllTagsOutput, Box<dyn Error>>) -> Self::View;
}

pub struct ListAllTagsController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> ListAllTagsController<T, P>
where
    T: TagRepository,
    P: ListAllTagsPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self) -> <P as ListAllTagsPresenter>::View {
        let result = ListAllTagsUseCase::new(self.repository)
            .exec()
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
