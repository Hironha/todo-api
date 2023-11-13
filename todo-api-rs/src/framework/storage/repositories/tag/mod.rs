pub(super) mod mapper;

use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};

use crate::application::repositories::tag::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::tag::delete::{Delete, DeleteError};
use crate::application::repositories::tag::find::{Find, FindError};
use crate::application::repositories::tag::list::{List, ListError};
use crate::application::repositories::tag::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

use mapper::{map_tag_model_to_entity, MapTagModelError};

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
        let create_q = r#"
            INSERT INTO tag (id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, created_at, updated_at
        "#;

        let tag_model = sqlx::query_as::<_, TagModel>(create_q)
            .bind(payload.id.into_uuid())
            .bind(payload.name.into_string())
            .bind(payload.description.into_opt_string())
            .bind(payload.created_at.into_date_time())
            .bind(payload.updated_at.into_date_time())
            .fetch_one(&self.pool)
            .await
            .map_err(CreateError::from_err)?;

        map_tag_model_to_entity(tag_model).map_err(CreateError::from_err)
    }
}

#[async_trait]
impl Delete for TagRepository {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let delete_q = "DELETE FROM tag WHERE id = $1";
        sqlx::query(delete_q)
            .bind(id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::from_err(err),
            })?;

        Ok(())
    }
}

#[async_trait]
impl Find for TagRepository {
    async fn find(&self, id: Id) -> Result<TagEntity, FindError> {
        let find_q = r#"
            SELECT id, name, description, created_at, updated_at
            FROM tag
            WHERE id = $1
        "#;

        let tag_model = sqlx::query_as::<_, TagModel>(find_q)
            .bind(id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => FindError::from_err(err),
            })?;

        map_tag_model_to_entity(tag_model).map_err(FindError::from_err)
    }
}

#[async_trait]
impl List for TagRepository {
    async fn list(&self) -> Result<Vec<TagEntity>, ListError> {
        let list_q = r#"
            SELECT id, name, description, created_at, updated_at
            FROM tag
        "#;

        let tags_model = sqlx::query_as::<_, TagModel>(list_q)
            .fetch_all(&self.pool)
            .await
            .map_err(ListError::from_err)?;

        tags_model
            .into_iter()
            .map(map_tag_model_to_entity)
            .collect::<Result<Vec<TagEntity>, MapTagModelError>>()
            .map_err(ListError::from_err)
    }
}

#[async_trait]
impl Update for TagRepository {
    async fn update(&self, payload: UpdatePayload) -> Result<TagEntity, UpdateError> {
        let update_q = r#"
            UPDATE tag
            SET name = $1, description = $2, updated_at = $3
            WHERE id = $4
            RETURNING id, name, description, created_at, updated_at
        "#;

        let tag_model = sqlx::query_as::<_, TagModel>(update_q)
            .bind(payload.name.into_string())
            .bind(payload.description.into_opt_string())
            .bind(payload.updated_at.into_date_time())
            .bind(payload.id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::from_err(err),
            })?;

        map_tag_model_to_entity(tag_model).map_err(UpdateError::from_err)
    }
}
