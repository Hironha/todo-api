use crate::adapters::dtos::tag::list::{RunError, TagList};
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::functions::tag::list::{list_tag, ListTagContext};
use crate::application::repositories::tag::TagRepository;

#[derive(Clone, Debug)]
pub struct ListController<T> {
    tag_repository: T,
}

impl<T> ListController<T>
where
    T: TagRepository,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run(&self) -> Result<TagList, RunError> {
        let ctx = ListTagContext {
            tag_repository: &self.tag_repository,
        };

        let tags = list_tag(ctx).await.map_err(RunError::Listing)?;

        let list = TagList {
            count: u64::try_from(tags.len()).unwrap_or(tags.len() as u64),
            items: tags.into_iter().map(TagPresenter::from).collect(),
        };

        Ok(list)
    }
}
