use crate::adapters::views::tag::TagView;
use crate::domain::entities::tag::TagEntity;

#[derive(Clone, Debug)]
pub struct Output(Result<Vec<TagView>, RunError>);

impl Output {
    pub const fn err(err: RunError) -> Self {
        Self(Err(err))
    }

    pub fn from_tags(tags: impl Into<Vec<TagEntity>>) -> Self {
        let tags: Vec<TagEntity> = tags.into();
        let views = tags
            .into_iter()
            .map(TagView::from)
            .collect::<Vec<TagView>>();

        Self(Ok(views))
    }

    pub fn into_result(self) -> Result<Vec<TagView>, RunError> {
        self.0
    }
}

#[derive(Clone, Debug)]
pub enum RunError {
    Internal,
}
