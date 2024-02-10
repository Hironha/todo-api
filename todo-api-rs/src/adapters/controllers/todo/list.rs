use crate::adapters::dtos::todo::list::{ListTodosPresenter, ListTodosRequest};
use crate::application::dtos::todo::list::{ListTodosInput, ListTodosOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct ListTodosController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> ListTodosController<T, P>
where
    T: UseCase<ListTodosInput, ListTodosOutput>,
    P: ListTodosPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: ListTodosRequest) -> <P as ListTodosPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(input).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
