use crate::adapters::dtos::tag::list::{RunError, TagList};
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::dtos::tag::list::ListTagError;
use crate::application::functions::tag::list::{list_tag, ListTagContext};
use crate::application::repositories::tag::list::List;

#[derive(Clone, Debug)]
pub struct ListController<Repo: List> {
    repository: Repo,
}

impl<Repo: List> ListController<Repo> {
    pub const fn new(repository: Repo) -> Self {
        Self { repository }
    }

    pub async fn run(&self) -> Result<TagList, RunError> {
        let ctx = ListTagContext::new(&self.repository);
        let tags = list_tag(ctx).await.into_result().map_err(|err| match err {
            ListTagError::Internal => RunError::Internal,
        })?;

        let list = TagList {
            count: u64::try_from(tags.len()).or(Err(RunError::Internal))?,
            items: tags.into_iter().map(TagPresenter::from).collect(),
        };

        Ok(list)
    }
}
