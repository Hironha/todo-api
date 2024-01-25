use std::error::Error;

use serde::Serialize;

use crate::adapters::controllers::tag::create::CreateTagPresenter;
use crate::adapters::controllers::tag::delete::DeleteTagPresenter;
use crate::adapters::controllers::tag::find::FindTagPresenter;
use crate::adapters::controllers::tag::list_all::ListAllTagsPresenter;
use crate::adapters::controllers::tag::update::UpdateTagPresenter;
use crate::adapters::views::tag::TagView;
use crate::application::dtos::tag::list_all::ListAllTagsOutput;
use crate::domain::entities::tag::TagEntity;

#[derive(Clone, Debug, Serialize)]
pub struct ListAllTagsView {
    items: Vec<TagView>,
    count: usize,
}

#[derive(Clone, Debug)]
pub struct JsonTagPresenter;

impl JsonTagPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl CreateTagPresenter for JsonTagPresenter {
    type View = Result<TagView, Box<dyn Error>>;

    fn present(&self, result: Result<TagEntity, Box<dyn Error>>) -> Self::View {
        result.map(TagView::from)
    }
}

impl DeleteTagPresenter for JsonTagPresenter {
    type View = Result<(), Box<dyn Error>>;

    fn present(&self, result: Result<(), Box<dyn Error>>) -> Self::View {
        result
    }
}

impl FindTagPresenter for JsonTagPresenter {
    type View = Result<TagView, Box<dyn Error>>;

    fn present(&self, result: Result<TagEntity, Box<dyn Error>>) -> Self::View {
        result.map(TagView::from)
    }
}

impl ListAllTagsPresenter for JsonTagPresenter {
    type View = Result<ListAllTagsView, Box<dyn Error>>;

    fn present(&self, result: Result<ListAllTagsOutput, Box<dyn Error>>) -> Self::View {
        let output = result?;
        let views: Vec<TagView> = output.items.into_iter().map(TagView::from).collect();

        Ok(ListAllTagsView {
            count: views.len(),
            items: views,
        })
    }
}

impl UpdateTagPresenter for JsonTagPresenter {
    type View = Result<TagView, Box<dyn Error>>;

    fn present(&self, result: Result<TagEntity, Box<dyn Error>>) -> Self::View {
        result.map(TagView::from)
    }
}
