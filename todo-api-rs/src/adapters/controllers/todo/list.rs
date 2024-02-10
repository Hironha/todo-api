use crate::adapters::dtos::todo::list::{ListTodosPresenter, ListTodosRequest};
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::list::ListTodosUseCase;

#[derive(Debug)]
pub struct ListTodosController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> ListTodosController<T, P>
where
    T: TodoRepository,
    P: ListTodosPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: ListTodosRequest) -> <P as ListTodosPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = ListTodosUseCase::new(self.repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
