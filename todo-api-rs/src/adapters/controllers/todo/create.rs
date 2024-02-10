use crate::adapters::dtos::todo::create::{CreateTodoPresenter, CreateTodoRequest};
use crate::application::dtos::todo::create::{CreateTodoInput, CreateTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct CreateTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> CreateTodoController<T, P>
where
    T: UseCase<CreateTodoInput, CreateTodoOutput>,
    P: CreateTodoPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: CreateTodoRequest) -> <P as CreateTodoPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(input).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
