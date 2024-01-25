use std::error::Error;

use crate::adapters::dtos::tag::create::CreateTagRequest;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::create::CreateTagUseCase;
use crate::domain::entities::tag::TagEntity;

pub trait CreateTagPresenter {
    type View;
    fn present(&self, result: Result<TagEntity, Box<dyn Error>>) -> Self::View;
}

pub struct CreateTagController<T, P>
where
    T: TagRepository,
    P: CreateTagPresenter,
{
    repository: T,
    presenter: P,
}

impl<T, P> CreateTagController<T, P>
where
    T: TagRepository,
    P: CreateTagPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: CreateTagRequest) -> <P as CreateTagPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = CreateTagUseCase::new(self.repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
