use std::error::Error;

use crate::adapters::dtos::tag::list::TagsList;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::list::ListTagsUseCase;

#[derive(Clone, Debug)]
pub struct ListController<T>
where
    T: TagRepository + Clone,
{
    tag_repository: T,
}

impl<T> ListController<T>
where
    T: TagRepository + Clone,
{
    pub const fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn run(&self) -> Result<TagsList, Box<dyn Error>> {
        let tags = ListTagsUseCase::new(self.tag_repository.clone())
            .exec()
            .await?;

        let list = TagsList {
            count: u64::try_from(tags.len()).unwrap_or(tags.len() as u64),
            items: tags.into_iter().map(TagPresenter::from).collect(),
        };

        Ok(list)
    }
}
