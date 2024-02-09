use std::error;

use crate::application::dtos::tag::list_all::ListAllTagsOutput;

pub trait ListAllTagsPresenter {
    type View;
    fn present(&self, result: Result<ListAllTagsOutput, Box<dyn error::Error>>) -> Self::View;
}
