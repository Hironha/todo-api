use crate::adapters::dtos::todo::delete::{
    DeletePresenter, DeleteRequest, DeleteResponseError,
};
use crate::application::dtos::todo::delete::{DeleteTodoError, DeleteTodoInput, DeleteTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct DeleteTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> DeleteTodoController<T, P>
where
    T: UseCase<DeleteTodoInput, DeleteTodoOutput>,
    P: DeletePresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: DeleteRequest) -> <P as DeletePresenter>::View {
        let todo_id = match req.parse().map_err(DeleteResponseError::Input) {
            Ok(todo_id) => todo_id,
            Err(err) => return self.presenter.present(Err(err)),
        };

        let result = self
            .interactor
            .exec(todo_id)
            .await
            .map_err(|err| match err {
                DeleteTodoError::NotFound => DeleteResponseError::NotFound(todo_id),
                DeleteTodoError::Internal(src) => DeleteResponseError::Internal(src),
            });

        self.presenter.present(result)
    }
}
