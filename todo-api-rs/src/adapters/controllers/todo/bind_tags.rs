use crate::adapters::dtos::todo::bind_tags::{BindTodoTagsPresenter, BindTodoTagsRequest};
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::bind_tags::BindTodoTagsUseCase;

#[derive(Debug)]
pub struct BindTodoTagsController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> BindTodoTagsController<T, P>
where
    T: TodoRepository,
    P: BindTodoTagsPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: BindTodoTagsRequest) -> <P as BindTodoTagsPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = BindTodoTagsUseCase::new(self.repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
