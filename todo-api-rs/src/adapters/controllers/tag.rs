use std::error::Error;

use crate::adapters::dtos::tag::create::CreateRequest;
use crate::adapters::dtos::tag::delete::DeleteRequest;
use crate::adapters::dtos::tag::find::FindRequest;
use crate::adapters::dtos::tag::list::ListAllResponse;
use crate::adapters::dtos::tag::update::UpdateRequest;
use crate::adapters::presenters::tag::TagPresenter;
use crate::application::repositories::tag::TagRepository;
use crate::application::use_cases::tag::create::CreateTagUseCase;
use crate::application::use_cases::tag::delete::DeleteTagUseCase;
use crate::application::use_cases::tag::find::FindTagUseCase;
use crate::application::use_cases::tag::list_all::ListAllTagsUseCase;
use crate::application::use_cases::tag::update::UpdateTagUseCase;

pub struct TagController<T>
where
    T: TagRepository,
{
    tag_repository: T,
}

impl<T> TagController<T>
where
    T: TagRepository,
{
    pub fn new(tag_repository: T) -> Self {
        Self { tag_repository }
    }

    pub async fn create(self, req: CreateRequest) -> Result<TagPresenter, Box<dyn Error>> {
        let input = req.parse()?;

        CreateTagUseCase::new(self.tag_repository)
            .exec(input)
            .await
            .map(TagPresenter::from)
            .map_err(Box::from)
    }

    pub async fn delete(self, req: DeleteRequest) -> Result<(), Box<dyn Error>> {
        let tag_id = req.parse()?;

        DeleteTagUseCase::new(self.tag_repository)
            .exec(tag_id)
            .await
            .map_err(Box::from)
    }

    pub async fn find(self, req: FindRequest) -> Result<TagPresenter, Box<dyn Error>> {
        let tag_id = req.parse()?;

        FindTagUseCase::new(self.tag_repository)
            .exec(tag_id)
            .await
            .map(TagPresenter::from)
            .map_err(Box::from)
    }

    pub async fn list(self) -> Result<ListAllResponse, Box<dyn Error>> {
        ListAllTagsUseCase::new(self.tag_repository)
            .exec()
            .await
            .map(ListAllResponse::from_tags)
            .map_err(Box::from)
    }

    pub async fn update(self, req: UpdateRequest) -> Result<TagPresenter, Box<dyn Error>> {
        let input = req.parse()?;

        UpdateTagUseCase::new(self.tag_repository)
            .exec(input)
            .await
            .map(TagPresenter::from)
            .map_err(Box::from)
    }
}
