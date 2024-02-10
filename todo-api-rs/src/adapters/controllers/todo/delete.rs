use crate::adapters::dtos::todo::delete::{DeleteTodoPresenter, DeleteTodoRequest};
use crate::application::dtos::todo::delete::{DeleteTodoInput, DeleteTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct DeleteTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> DeleteTodoController<T, P>
where
    T: UseCase<DeleteTodoInput, DeleteTodoOutput>,
    P: DeleteTodoPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: DeleteTodoRequest) -> <P as DeleteTodoPresenter>::View {
        let todo_id = match req.parse() {
            Ok(todo_id) => todo_id,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(todo_id).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
