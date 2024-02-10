use std::error;

use sqlx::types::uuid::Uuid;
use sqlx::{Error as SqlxError, PgPool};

use crate::application::repositories::tag::{
    CreateError, DeleteError, ExistsManyError, FindError, ListAllError, TagRepository, UpdateError,
};
use crate::domain::entities::tag::TagEntity;
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;

#[derive(Clone)]
pub struct PgTagRepository {
    pool: PgPool,
}

impl PgTagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TagRepository for PgTagRepository {
    async fn create(&mut self, tag: TagEntity) -> Result<TagEntity, CreateError> {
        let create_q = r#"
            INSERT INTO tag (id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, created_at, updated_at
        "#;

        let name = tag.name.into_inner();
        let tag_model = sqlx::query_as::<_, TagModel>(create_q)
            .bind(tag.id.uuid())
            .bind(name.as_str())
            .bind(tag.description.map(|d| d.into_inner()))
            .bind(tag.created_at.date_time())
            .bind(tag.updated_at.date_time())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                    CreateError::DuplicatedName
                }
                _ => CreateError::Internal(err.into()),
            })?;

        tag_model.try_into_entity().map_err(CreateError::Internal)
    }

    async fn delete(&mut self, tag_id: Id) -> Result<(), DeleteError> {
        let delete_q = "DELETE FROM tag WHERE id = $1 RETURNING id";
        sqlx::query(delete_q)
            .bind(tag_id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::Internal(err.into()),
            })?;

        Ok(())
    }

    async fn exists_many(&self, tag_ids: &[Id]) -> Result<(), ExistsManyError> {
        let tag_uuids = tag_ids.iter().map(|id| id.uuid()).collect::<Vec<Uuid>>();

        let select_any_q = "SELECT id FROM tag WHERE id = ANY($1)";
        let selected_tag_uuids = sqlx::query_scalar::<_, Uuid>(select_any_q)
            .bind(&tag_uuids)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ExistsManyError::Internal(e.into()))?;

        let not_found_ids = tag_uuids
            .into_iter()
            .filter(|uuid| !selected_tag_uuids.contains(uuid))
            .map(Id::from)
            .collect::<Vec<Id>>();

        if not_found_ids.is_empty() {
            Ok(())
        } else {
            Err(ExistsManyError::NotFound(not_found_ids))
        }
    }

    async fn find(&self, tag_id: Id) -> Result<TagEntity, FindError> {
        let find_q = r#"
            SELECT id, name, description, created_at, updated_at
            FROM tag
            WHERE id = $1
        "#;

        let tag_model = sqlx::query_as::<_, TagModel>(find_q)
            .bind(tag_id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => FindError::Internal(err.into()),
            })?;

        tag_model.try_into_entity().map_err(FindError::Internal)
    }

    async fn list_all(&self) -> Result<Vec<TagEntity>, ListAllError> {
        let list_all_q = r#"
            SELECT id, name, description, created_at, updated_at
            FROM tag
        "#;

        let tag_models = sqlx::query_as::<_, TagModel>(list_all_q)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ListAllError::Internal(e.into()))?;

        tag_models
            .into_iter()
            .map(|model| model.try_into_entity())
            .collect::<Result<Vec<TagEntity>, Box<dyn error::Error>>>()
            .map_err(ListAllError::Internal)
    }

    async fn update(&mut self, tag: TagEntity) -> Result<(), UpdateError> {
        let update_q = r#"
            UPDATE tag
            SET name = $1, description = $2, updated_at = $3
            WHERE id = $4
        "#;

        sqlx::query_scalar(update_q)
            .bind(tag.name.into_inner())
            .bind(tag.description.map(|d| d.into_inner()))
            .bind(tag.updated_at.date_time())
            .bind(tag.id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                    UpdateError::DuplicatedName
                }
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::Internal(err.into()),
            })?;

        Ok(())
    }
}
