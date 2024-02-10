use crate::adapters::dtos::tag::find::{FindTagPresenter, FindTagRequest};
use crate::application::dtos::tag::find::{FindTagInput, FindTagOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct FindTagController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> FindTagController<T, P>
where
    T: UseCase<FindTagInput, FindTagOutput>,
    P: FindTagPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: FindTagRequest) -> <P as FindTagPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(input).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
