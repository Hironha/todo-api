use async_trait::async_trait;
use sqlx::{Error as SqlxError, Pool, Postgres};
use time::OffsetDateTime;

use crate::application::repositories::tag::create::{Create, CreateError, CreateTagPayload};
use crate::application::repositories::tag::delete::{Delete, DeleteError};
use crate::application::repositories::tag::find::{Find, FindError};
use crate::application::repositories::tag::list::{List, ListError};
use crate::application::repositories::tag::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::tag::{Description, Name, TagEntity};
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

#[derive(Clone)]
pub struct TagStore {
    pool: Pool<Postgres>,
}

impl TagStore {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Create for TagStore {
    async fn create(&self, payload: CreateTagPayload) -> Result<TagEntity, CreateError> {
        let q = r#"
            INSERT INTO tag (id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, created_at, updated_at
        "#;

        let current_date_time = OffsetDateTime::now_utc();
        let model = sqlx::query_as::<_, TagModel>(q)
            .bind(Id::new().into_uuid())
            .bind(payload.name.into_string())
            .bind(payload.description.into_opt_string())
            .bind(current_date_time)
            .bind(current_date_time)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("create tag repository error {err:?}");
                CreateError::Internal
            })?;

        tag_model_to_entity(model).map_err(|_| CreateError::Internal)
    }
}

#[async_trait]
impl Find for TagStore {
    async fn find(&self, id: Id) -> Result<TagEntity, FindError> {
        let q = r#"
            SELECT id, name, description, created_at, updated_at
            FROM tag
            WHERE id = ($1)
        "#;

        let model = sqlx::query_as::<_, TagModel>(q)
            .bind(id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => {
                    tracing::error!("find tag repository error {err:?}");
                    FindError::Internal
                }
            })?;

        tag_model_to_entity(model).map_err(|_| FindError::Internal)
    }
}

#[async_trait]
impl Delete for TagStore {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let q = r#"
            DELETE FROM tag
            WHERE id = ($1)
        "#;

        sqlx::query(q)
            .bind(id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map(|_| ())
            .map_err(|err| match err {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => {
                    tracing::error!("delete tag repository error {err:?}");
                    DeleteError::Internal
                }
            })
    }
}

#[async_trait]
impl List for TagStore {
    async fn list(&self) -> Result<Vec<TagEntity>, ListError> {
        let q = r#"
            SELECT id, name, description, created_at, updated_at
            FROM tag
        "#;

        let tag_models = sqlx::query_as::<_, TagModel>(q)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| ListError::Internal)?;

        tag_models
            .into_iter()
            .map(tag_model_to_entity)
            .collect::<Result<Vec<TagEntity>, ()>>()
            .map_err(|_| ListError::Internal)
    }
}

#[async_trait]
impl Update for TagStore {
    async fn update(&self, payload: UpdatePayload) -> Result<TagEntity, UpdateError> {
        let q = r#"
            UPDATE tag
            SET name = ($1), description = ($2)
            WHERE id = ($3)
            RETURNING id, name, description, created_at, updated_at
        "#;

        let model = sqlx::query_as::<_, TagModel>(q)
            .bind(payload.id.into_uuid())
            .bind(payload.name.into_string())
            .bind(payload.description.into_opt_string())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => {
                    tracing::error!("update tag repository error {err:?}");
                    UpdateError::Internal
                }
            })?;

        tag_model_to_entity(model).map_err(|_| UpdateError::Internal)
    }
}

fn tag_model_to_entity(model: TagModel) -> Result<TagEntity, ()> {
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
