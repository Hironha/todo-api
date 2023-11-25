use async_trait::async_trait;
use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::{Error as SqlxError, PgPool};

use crate::application::repositories::tag::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::tag::delete::{Delete, DeleteError};
use crate::application::repositories::tag::exists_all::{ExistsAll, ExistsAllError};
use crate::application::repositories::tag::find::{Find, FindError};
use crate::application::repositories::tag::list::{List, ListError};
use crate::application::repositories::tag::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

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
impl ExistsAll for TagRepository {
    async fn exists_all(&self, tags_id: &[Id]) -> Result<(), ExistsAllError> {
        let tags_uuid = tags_id
            .iter()
            .map(|id| id.into_uuid())
            .collect::<Vec<Uuid>>();

        let select_any_q = "SELECT id FROM tag WHERE id = ANY($1)";
        let selected_tags_uuid = sqlx::query_scalar::<_, Uuid>(select_any_q)
            .bind(&tags_uuid)
            .fetch_all(&self.pool)
            .await
            .map_err(ExistsAllError::from_err)?;

        let not_found_ids = tags_uuid
            .into_iter()
            .filter(|uuid| !selected_tags_uuid.contains(uuid))
            .map(Id::from)
            .collect::<Vec<Id>>();

        if not_found_ids.is_empty() {
            Ok(())
        } else {
            Err(ExistsAllError::NotFound(not_found_ids))
        }
    }
}

#[async_trait]
impl Create for TagRepository {
    async fn create(&self, payload: CreatePayload) -> Result<TagEntity, CreateError> {
        let current_dt = OffsetDateTime::now_utc();
        let create_q = r#"
            INSERT INTO tag (id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, created_at, updated_at
        "#;

        let tag_model = sqlx::query_as::<_, TagModel>(create_q)
            .bind(Id::new().into_uuid())
            .bind(payload.name.into_string())
            .bind(payload.description.map(|d| d.into_string()))
            .bind(current_dt)
            .bind(current_dt)
            .fetch_one(&self.pool)
            .await
            .map_err(CreateError::from_err)?;

        tag_model.try_into_entity().map_err(CreateError::from_err)
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

        tag_model.try_into_entity().map_err(FindError::from_err)
    }
}

#[async_trait]
impl List for TagRepository {
    async fn list(&self) -> Result<Vec<TagEntity>, ListError> {
        let list_q = r#"
            SELECT id, name, description, created_at, updated_at
            FROM tag
        "#;

        let tag_models = sqlx::query_as::<_, TagModel>(list_q)
            .fetch_all(&self.pool)
            .await
            .map_err(ListError::from_err)?;

        tag_models
            .into_iter()
            .map(|model| model.try_into_entity().map_err(ListError::from_err))
            .collect::<Result<Vec<TagEntity>, ListError>>()
    }
}

#[async_trait]
impl Update for TagRepository {
    async fn update(&self, payload: UpdatePayload) -> Result<TagEntity, UpdateError> {
        let current_dt = OffsetDateTime::now_utc();
        let update_q = r#"
            UPDATE tag
            SET name = $1, description = $2, updated_at = $3
            WHERE id = $4
            RETURNING id, name, description, created_at, updated_at
        "#;

        let tag_model = sqlx::query_as::<_, TagModel>(update_q)
            .bind(payload.name.into_string())
            .bind(payload.description.map(|d| d.into_string()))
            .bind(current_dt)
            .bind(payload.id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::from_err(err),
            })?;

        tag_model.try_into_entity().map_err(UpdateError::from_err)
    }
}
