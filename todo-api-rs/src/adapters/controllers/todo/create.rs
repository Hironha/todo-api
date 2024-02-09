use std::error::Error;

use crate::adapters::dtos::todo::create::CreateTodoRequest;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::create::CreateTodoUseCase;
use crate::domain::entities::todo::TodoEntity;

pub trait CreateTodoPresenter {
    type View;
    fn present(&self, result: Result<TodoEntity, Box<dyn Error>>) -> Self::View;
}

pub struct CreateTodoController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> CreateTodoController<T, P>
where
    T: TodoRepository,
    P: CreateTodoPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: CreateTodoRequest) -> <P as CreateTodoPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = CreateTodoUseCase::new(self.repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
