use crate::adapters::dtos::todo::update::{UpdateTodoPresenter, UpdateTodoRequest};
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::update::UpdateTodoUseCase;

#[derive(Debug)]
pub struct UpdateTodoController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> UpdateTodoController<T, P>
where
    T: TodoRepository,
    P: UpdateTodoPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: UpdateTodoRequest) -> <P as UpdateTodoPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = UpdateTodoUseCase::new(self.repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
