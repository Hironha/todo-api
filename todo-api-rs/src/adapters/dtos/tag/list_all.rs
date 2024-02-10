use std::error;

use crate::application::dtos::tag::list_all::AllTagsList;

pub trait ListAllTagsPresenter {
    type View;
    fn present(&self, result: Result<AllTagsList, Box<dyn error::Error>>) -> Self::View;
}
