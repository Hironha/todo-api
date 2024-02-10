use crate::adapters::dtos::tag::delete::{DeleteTagPresenter, DeleteTagRequest};
use crate::application::dtos::tag::delete::{DeleteTagInput, DeleteTagOutput};
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct DeleteTagController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> DeleteTagController<T, P>
where
    T: UseCase<DeleteTagInput, DeleteTagOutput>,
    P: DeleteTagPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self, req: DeleteTagRequest) -> <P as DeleteTagPresenter>::View {
        let tag_id = match req.parse() {
            Ok(tag_id) => tag_id,
            Err(err) => return self.presenter.present(Err(err.into())),
        };

        let result = self.interactor.exec(tag_id).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
