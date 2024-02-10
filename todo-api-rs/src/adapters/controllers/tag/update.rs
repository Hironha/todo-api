use crate::adapters::dtos::tag::update::{UpdateTagPresenter, UpdateTagRequest};
use crate::application::dtos::tag::update::{UpdateTagInput, UpdateTagOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct UpdateTagController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> UpdateTagController<T, P>
where
    T: UseCase<UpdateTagInput, UpdateTagOutput>,
    P: UpdateTagPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: UpdateTagRequest) -> <P as UpdateTagPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(input).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
