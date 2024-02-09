use std::error::Error;

use crate::adapters::dtos::tag::update::UpdateTagRequest;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::update::UpdateTagUseCase;
use crate::domain::entities::tag::TagEntity;

pub trait UpdateTagPresenter {
    type View;
    fn present(&self, result: Result<TagEntity, Box<dyn Error>>) -> Self::View;
}

pub struct UpdateTagController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> UpdateTagController<T, P>
where
    T: TagRepository,
    P: UpdateTagPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: UpdateTagRequest) -> <P as UpdateTagPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = UpdateTagUseCase::new(self.repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
