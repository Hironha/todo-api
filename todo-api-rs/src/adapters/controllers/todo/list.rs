use crate::adapters::dtos::todo::list::{ListPresenter, ListRequest, ListResponseError};
use crate::application::dtos::todo::list::{ListTodosError, ListTodosInput, ListTodosOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct ListTodosController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> ListTodosController<T, P>
where
    T: UseCase<ListTodosInput, ListTodosOutput>,
    P: ListPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: ListRequest) -> <P as ListPresenter>::View {
        let input = match req.parse().map_err(ListResponseError::Input) {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err)),
        };

        let result = self.interactor.exec(input).await.map_err(|err| match err {
            ListTodosError::Internal(src) => ListResponseError::Internal(src),
        });

        self.presenter.present(result)
    }
}
