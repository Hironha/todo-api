use std::error::Error;

use crate::adapters::dtos::tag::find::FindTagRequest;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::find::FindTagUseCase;
use crate::domain::entities::tag::TagEntity;

pub trait FindTagPresenter {
    type View;
    fn present(&self, result: Result<TagEntity, Box<dyn Error>>) -> Self::View;
}

pub struct FindTagController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> FindTagController<T, P>
where
    T: TagRepository,
    P: FindTagPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: FindTagRequest) -> <P as FindTagPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = FindTagUseCase::new(self.repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
