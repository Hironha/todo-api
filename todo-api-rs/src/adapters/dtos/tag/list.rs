use serde::Serialize;

use crate::{adapters::presenters::tag::TagPresenter, domain::entities::tag::TagEntity};

#[derive(Clone, Debug, Serialize)]
pub struct ListAllResponse {
    pub items: Vec<TagPresenter>,
    pub count: u64,
}

impl ListAllResponse {
    pub fn from_tags(tags: Vec<TagEntity>) -> ListAllResponse {
        Self {
            count: u64::try_from(tags.len()).unwrap_or(tags.len() as u64),
            items: tags.into_iter().map(TagPresenter::from).collect(),
        }
    }
}
