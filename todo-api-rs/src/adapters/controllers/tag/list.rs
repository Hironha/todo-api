use crate::adapters::dtos::tag::list::{RunError, TagList};
use crate::adapters::presenters::tag::TagPresenter;
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
        let tags = list_tag(ctx).await.map_err(RunError::Listing)?;

        let list = TagList {
            count: u64::try_from(tags.len()).unwrap_or(tags.len() as u64),
            items: tags.into_iter().map(TagPresenter::from).collect(),
        };

        Ok(list)
    }
}
