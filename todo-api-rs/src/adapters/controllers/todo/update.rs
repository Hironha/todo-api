use crate::adapters::dtos::todo::update::{UpdatePresenter, UpdateRequest, UpdateResponseError};
use crate::application::dtos::todo::update::{UpdateTodoError, UpdateTodoInput, UpdateTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct UpdateTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> UpdateTodoController<T, P>
where
    T: UseCase<UpdateTodoInput, UpdateTodoOutput>,
    P: UpdatePresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: UpdateRequest) -> <P as UpdatePresenter>::View {
        let input = match req.parse().map_err(UpdateResponseError::Input) {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err)),
        };

        let todo_id = input.id;
        let result = self.interactor.exec(input).await.map_err(|err| match err {
            UpdateTodoError::NotFound => UpdateResponseError::NotFound(todo_id),
            UpdateTodoError::DuplicatedTitle(title) => UpdateResponseError::DuplicatedTitle(title),
            UpdateTodoError::Internal(src) => UpdateResponseError::Internal(src),
        });

        self.presenter.present(result)
    }
}
