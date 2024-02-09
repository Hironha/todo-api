use crate::adapters::dtos::todo::delete::{DeleteTodoPresenter, DeleteTodoRequest};
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::delete::DeleteTodoUseCase;

pub struct DeleteTodoController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> DeleteTodoController<T, P>
where
    T: TodoRepository,
    P: DeleteTodoPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: DeleteTodoRequest) -> <P as DeleteTodoPresenter>::View {
        let todo_id = match req.parse() {
            Ok(todo_id) => todo_id,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = DeleteTodoUseCase::new(self.repository)
            .exec(todo_id)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
