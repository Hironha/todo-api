use crate::adapters::dtos::tag::list_all::ListAllTagsPresenter;
use crate::application::dtos::tag::list_all::ListAllTagsOutput;
use crate::domain::use_case::UseCase;

#[derive(Debug)]
pub struct ListAllTagsController<T, P> {
    interactor: T,
    presenter: P,
}

impl<T, P> ListAllTagsController<T, P>
where
    T: UseCase<(), ListAllTagsOutput>,
    P: ListAllTagsPresenter,
{
    pub const fn new(interactor: T, presenter: P) -> Self {
        Self {
            interactor,
            presenter,
        }
    }

    pub async fn run(self) -> <P as ListAllTagsPresenter>::View {
        let result = self.interactor.exec(()).await.map_err(Box::from);
        self.presenter.present(result)
    }
}
