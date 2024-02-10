use crate::adapters::dtos::todo::update::{UpdateTodoPresenter, UpdateTodoRequest};
use crate::application::dtos::todo::update::{UpdateTodoInput, UpdateTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct UpdateTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> UpdateTodoController<T, P>
where
    T: UseCase<UpdateTodoInput, UpdateTodoOutput>,
    P: UpdateTodoPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: UpdateTodoRequest) -> <P as UpdateTodoPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(input).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
