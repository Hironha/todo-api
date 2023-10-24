mod create;
mod delete;
mod find;
mod list;
mod update;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::application::repositories::tag::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::tag::delete::{Delete, DeleteError};
use crate::application::repositories::tag::find::{Find, FindError};
use crate::application::repositories::tag::list::{List, ListError};
use crate::application::repositories::tag::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

use create::create_tag;
use delete::delete_tag;
use find::find_tag;
use list::list_tag;
use update::update_tag;

#[derive(Clone)]
pub struct TagRepository {
    pool: PgPool,
}

impl TagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Create for TagRepository {
    async fn create(&self, payload: CreatePayload) -> Result<TagEntity, CreateError> {
        let mut conn = self.pool.acquire().await.or(Err(CreateError::Internal))?;
        let model = create_tag(conn.as_mut(), payload).await?;
        map_tag_model_to_entity(model).or(Err(CreateError::Internal))
    }
}

#[async_trait]
impl Delete for TagRepository {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let mut conn = self.pool.acquire().await.or(Err(DeleteError::Internal))?;
        delete_tag(conn.as_mut(), id).await
    }
}

#[async_trait]
impl Find for TagRepository {
    async fn find(&self, id: Id) -> Result<TagEntity, FindError> {
        let mut conn = self.pool.acquire().await.or(Err(FindError::Internal))?;
        let model = find_tag(conn.as_mut(), id).await?;
        map_tag_model_to_entity(model).or(Err(FindError::Internal))
    }
}

#[async_trait]
impl List for TagRepository {
    async fn list(&self) -> Result<Vec<TagEntity>, ListError> {
        let mut conn = self.pool.acquire().await.or(Err(ListError::Internal))?;
        let tag_models = list_tag(conn.as_mut()).await?;

        tag_models
            .into_iter()
            .map(map_tag_model_to_entity)
            .collect::<Result<Vec<TagEntity>, ()>>()
            .or(Err(ListError::Internal))
    }
}

#[async_trait]
impl Update for TagRepository {
    async fn update(&self, payload: UpdatePayload) -> Result<TagEntity, UpdateError> {
        let mut conn = self.pool.acquire().await.or(Err(UpdateError::Internal))?;
        let tag_model = update_tag(conn.as_mut(), payload).await?;
        map_tag_model_to_entity(tag_model).or(Err(UpdateError::Internal))
    }
}

fn map_tag_model_to_entity(model: TagModel) -> Result<TagEntity, ()> {
    let name = Name::new(model.name).map_err(|err| {
        let msg = err.description();
        tracing::error!("tag model name incompatible with tag entity name: {msg}");
    })?;
    let description = Description::new(model.description).map_err(|err| {
        let msg = err.description();
        tracing::error!("tag model description incompatible with tag entity description: {msg}");
    })?;

    Ok(TagEntity {
        id: model.id.into(),
        name,
        description,
        created_at: model.created_at.into(),
        updated_at: model.updated_at.into(),
    })
}
