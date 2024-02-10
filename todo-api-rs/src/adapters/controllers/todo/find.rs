use crate::adapters::dtos::todo::find::{FindTodoPresenter, FindTodoRequest};
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::find::FindTodoUseCase;

#[derive(Debug)]
pub struct FindTodoController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> FindTodoController<T, P>
where
    T: TodoRepository,
    P: FindTodoPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: FindTodoRequest) -> <P as FindTodoPresenter>::View {
        let todo_id = match req.parse() {
            Ok(todo_id) => todo_id,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = FindTodoUseCase::new(self.repository)
            .exec(todo_id)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
