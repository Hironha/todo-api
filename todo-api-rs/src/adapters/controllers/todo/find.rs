use crate::adapters::dtos::todo::find::{FindPresenter, FindRequest, FindResponseError};
use crate::application::dtos::todo::find::{FindTodoError, FindTodoInput, FindTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct FindTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> FindTodoController<T, P>
where
    T: UseCase<FindTodoInput, FindTodoOutput>,
    P: FindPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: FindRequest) -> <P as FindPresenter>::View {
        let todo_id = match req.parse().map_err(FindResponseError::Input) {
            Ok(todo_id) => todo_id,
            Err(err) => return self.presenter.present(Err(err)),
        };

        let result = self
            .interactor
            .exec(todo_id)
            .await
            .map_err(|err| match err {
                FindTodoError::NotFound => FindResponseError::NotFound(todo_id),
                FindTodoError::Internal(src) => FindResponseError::Internal(src),
            });

        self.presenter.present(result)
    }
}
