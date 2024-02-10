use crate::adapters::dtos::todo::find::{FindTodoPresenter, FindTodoRequest};
use crate::application::dtos::todo::find::{FindTodoInput, FindTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct FindTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> FindTodoController<T, P>
where
    T: UseCase<FindTodoInput, FindTodoOutput>,
    P: FindTodoPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: FindTodoRequest) -> <P as FindTodoPresenter>::View {
        let todo_id = match req.parse() {
            Ok(todo_id) => todo_id,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(todo_id).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
