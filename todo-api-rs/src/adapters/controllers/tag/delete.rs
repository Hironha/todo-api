use std::error::Error;

use crate::adapters::dtos::tag::delete::DeleteTagRequest;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::delete::DeleteTagUseCase;

pub trait DeleteTagPresenter {
    type View;
    fn present(&self, result: Result<(), Box<dyn Error>>) -> Self::View;
}

pub struct DeleteTagController<T, P> {
    repository: T,
    presenter: P,
}

impl<T, P> DeleteTagController<T, P>
where
    T: TagRepository,
    P: DeleteTagPresenter,
{
    pub const fn new(repository: T, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub async fn run(self, req: DeleteTagRequest) -> <P as DeleteTagPresenter>::View {
        let tag_id = match req.parse() {
            Ok(tag_id) => tag_id,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = DeleteTagUseCase::new(self.repository)
            .exec(tag_id)
            .await
            .map_err(Box::from);

        self.presenter.present(result)
    }
}
