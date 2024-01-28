use std::error::Error;

use crate::adapters::dtos::todo::bind_tags::BindTodoTagsRequest;
use crate::application::repositories::tag::TagRepository;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::bind_tags::BindTodoTagsUseCase;

pub trait BindTodoTagsPresenter {
    type View;
    fn present(&self, result: Result<(), Box<dyn Error>>) -> Self::View;
}

pub struct BindTodoTagsController<T, R, P>
where
    T: TodoRepository,
    R: TagRepository,
    P: BindTodoTagsPresenter,
{
    todo_repository: T,
    tag_repository: R,
    presenter: P,
}

impl<T, R, P> BindTodoTagsController<T, R, P>
where
    T: TodoRepository,
    R: TagRepository,
    P: BindTodoTagsPresenter,
{
    pub const fn new(todo_repository: T, tag_repository: R, presenter: P) -> Self {
        Self {
            todo_repository,
            tag_repository,
            presenter,
        }
    }

    pub async fn run(self, req: BindTodoTagsRequest) -> <P as BindTodoTagsPresenter>::View {
        let input = match req.parse() {
            Ok(input) => input,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = BindTodoTagsUseCase::new(self.todo_repository, self.tag_repository)
            .exec(input)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
