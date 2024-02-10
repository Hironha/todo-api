use crate::adapters::dtos::todo::bind_tags::{BindTodoTagsPresenter, BindTodoTagsRequest};
use crate::application::dtos::todo::bind_tags::{BindTodoTagsInput, BindTodoTagsOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct BindTodoTagsController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> BindTodoTagsController<T, P>
where
    T: UseCase<BindTodoTagsInput, BindTodoTagsOutput>,
    P: BindTodoTagsPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: BindTodoTagsRequest) -> <P as BindTodoTagsPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(input).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
