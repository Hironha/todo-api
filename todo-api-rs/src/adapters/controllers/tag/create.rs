use crate::adapters::dtos::tag::create::{CreateTagPresenter, CreateTagRequest};
use crate::application::dtos::tag::create::{CreateTagInput, CreateTagOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct CreateTagController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> CreateTagController<T, P>
where
    T: UseCase<CreateTagInput, CreateTagOutput>,
    P: CreateTagPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: CreateTagRequest) -> <P as CreateTagPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(input).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
