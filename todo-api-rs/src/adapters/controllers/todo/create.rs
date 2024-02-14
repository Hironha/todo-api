use crate::adapters::dtos::todo::create::{
    CreatePresenter, CreateRequest, CreateResponseError,
};
use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput, CreateTodoOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct CreateTodoController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> CreateTodoController<T, P>
where
    T: UseCase<CreateTodoInput, CreateTodoOutput>,
    P: CreatePresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: CreateRequest) -> <P as CreatePresenter>::View {
        let input = match req.parse().map_err(CreateResponseError::Input) {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err)),
        };

        let result = self.interactor.exec(input).await.map_err(|err| match err {
            CreateTodoError::DuplicatedTitle(title) => {
                CreateResponseError::DuplicatedTitle(title)
            }
            CreateTodoError::Internal(src) => CreateResponseError::Internal(src),
        });

        self.presenter.present(result)
    }
}
