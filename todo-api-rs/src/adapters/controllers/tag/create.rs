use crate::adapters::dtos::tag::create::{CreateTagPresenter, CreateTagRequest};
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::create::CreateTagUseCase;

pub struct CreateTagController<T, P> {
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
