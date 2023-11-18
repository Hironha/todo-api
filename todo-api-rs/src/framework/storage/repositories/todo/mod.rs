use async_trait::async_trait;
use sqlx::types::uuid::Uuid;
use sqlx::{Error as SqlxError, FromRow, PgPool, Postgres, QueryBuilder, Row};

use crate::application::repositories::todo::bind_tags::{BindTags, BindTagsError, BindTagsPayload};
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::application::repositories::todo::find::{Find, FindError};
use crate::application::repositories::todo::list::{List, ListData, ListError, ListPayload};
use crate::application::repositories::todo::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::tag::TagEntity;
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::Id;
use crate::framework::storage::models::tag::TagModel;
use crate::framework::storage::models::todo::TodoModel;

#[derive(Clone)]
pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BindTags for TodoRepository {
    async fn bind_tags(&self, payload: BindTagsPayload) -> Result<(), BindTagsError> {
        let mut trx = self.pool.begin().await.map_err(BindTagsError::from_err)?;
        let todo_id = payload.todo_id.into_uuid();
        let todo_exists_q = "SELECT EXISTS(SELECT 1 FROM todo WHERE id = $1)";
        let todo_exists = sqlx::query_scalar::<_, bool>(todo_exists_q)
            .bind(todo_id)
            .fetch_one(trx.as_mut())
            .await
            .map_err(BindTagsError::from_err)?;

        if !todo_exists {
            return Err(BindTagsError::TodoNotFound);
        }

        let delete_relations_q = "DELETE FROM todo_tag WHERE todo_id = $1";
        sqlx::query(delete_relations_q)
            .bind(todo_id)
            .execute(trx.as_mut())
            .await
            .map_err(BindTagsError::from_err)?;

        let tag_ids = payload.tags_id.filter(|ids| !ids.is_empty()).map(|ids| {
            ids.into_iter()
                .map(|id| id.into_uuid())
                .collect::<Vec<Uuid>>()
        });

        if let Some(tags_id) = tag_ids.as_deref() {
            let count_tags_q = "SELECT COUNT(*) FROM tag WHERE id = ANY($1)";
            let count = sqlx::query_scalar::<_, i64>(count_tags_q)
                .bind(tags_id)
                .fetch_one(trx.as_mut())
                .await
                .map_err(BindTagsError::from_err)?;

            let tags_exists = i64::try_from(tags_id.len()).unwrap_or(0) == count;
            if !tags_exists {
                return Err(BindTagsError::TagNotFound);
            }

            let current_dt = payload.current_dt.into_date_time();
            let base_bind_tags_q = "INSERT INTO todo_tag (todo_id, tag_id, created_at) ";
            QueryBuilder::<'_, Postgres>::new(base_bind_tags_q)
                .push_values(tags_id, |mut q, tag_id| {
                    q.push_bind(todo_id).push_bind(tag_id).push_bind(current_dt);
                })
                .build()
                .execute(trx.as_mut())
                .await
                .map_err(BindTagsError::from_err)?;
        }

        trx.commit().await.map_err(BindTagsError::from_err)
    }
}

#[async_trait]
impl Create for TodoRepository {
    async fn create(&self, payload: CreatePayload) -> Result<TodoEntity, CreateError> {
        let insert_q = r#"
            INSERT INTO todo (id, title, description, todo_at, done, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING todo.*
        "#;

        let todo_model = sqlx::query_as::<_, TodoModel>(insert_q)
            .bind(Id::new().into_uuid())
            .bind(payload.title.into_string())
            .bind(payload.description.map(|d| d.into_string()))
            .bind(payload.todo_at.map(|at| at.into_date()))
            .bind(payload.done)
            .bind(payload.created_at.into_date_time())
            .bind(payload.updated_at.into_date_time())
            .fetch_one(&self.pool)
            .await
            .map_err(CreateError::from_err)?;

        todo_model
            .try_into_entity(Vec::new())
            .map_err(CreateError::from_err)
    }
}

#[async_trait]
impl Delete for TodoRepository {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let delete_q = "DELETE FROM todo WHERE id = $1 RETURNING id";
        sqlx::query_scalar::<_, Uuid>(delete_q)
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
impl Find for TodoRepository {
    async fn find(&self, id: Id) -> Result<TodoEntity, FindError> {
        let todo_id = id.into_uuid();
        let find_todo_q = "SELECT todo.* FROM todo WHERE id = $1";
        let todo_model = sqlx::query_as::<_, TodoModel>(find_todo_q)
            .bind(todo_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => FindError::from_err(err),
            })?;

        let get_related_tags_q = r#"
            SELECT tag.*
            FROM tag
            INNER JOIN todo_tag ON todo_tag.tag_id = tag.id
            WHERE todo_tag.todo_id = $1
        "#;

        let todo_tag_models = sqlx::query_as::<_, TagModel>(get_related_tags_q)
            .bind(todo_id)
            .fetch_all(&self.pool)
            .await
            .map_err(FindError::from_err)?;

        let todo_tags = todo_tag_models
            .into_iter()
            .map(|model| model.try_into_entity().map_err(FindError::from_err))
            .collect::<Result<Vec<TagEntity>, FindError>>()?;

        todo_model
            .try_into_entity(todo_tags)
            .map_err(FindError::from_err)
    }
}

#[async_trait]
impl List for TodoRepository {
    // TODO: try to implement a database function to list with filters
    async fn list(&self, payload: ListPayload) -> Result<ListData, ListError> {
        let base_count_q = "SELECT COUNT(*) FROM todo";
        let base_list_q = "SELECT todo.* FROM todo";
        let mut count_q = QueryBuilder::<'_, Postgres>::new(base_count_q);
        let mut list_q = QueryBuilder::<'_, Postgres>::new(base_list_q);

        let title_filter = payload.title.as_ref().map(|t| format!("%{}%", t.as_str()));
        if let Some(constraint) = title_filter.as_deref() {
            count_q.push(" WHERE title ILIKE ").push_bind(constraint);
            list_q.push(" WHERE title ILIKE ").push_bind(constraint);
        }

        let count = count_q
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await
            .map_err(ListError::from_err)?;

        let limit: i64 = u32::from(payload.per_page).into();
        let page: i64 = u32::from(payload.page).into();
        let offset = (page - 1) * limit;

        let todo_models = list_q
            .push(" ORDER BY created_at DESC LIMIT ")
            .push_bind(limit)
            .push(" OFFSET ")
            .push_bind(offset)
            .build_query_as::<TodoModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(ListError::from_err)?;

        let todo_ids = todo_models
            .iter()
            .map(|todo| todo.id)
            .collect::<Vec<Uuid>>();

        let find_tags_q = r#"
            SELECT todo_id, tag.*
            FROM todo_tag
            INNER JOIN tag ON tag.id = todo_tag.tag_id
            WHERE todo_tag.todo_id = ANY($1) 
        "#;

        let tag_relations = sqlx::query(find_tags_q)
            .bind(&todo_ids)
            .fetch_all(&self.pool)
            .await
            .map_err(ListError::from_err)?;

        let tag_relation_entries = tag_relations
            .into_iter()
            .map(|row| {
                let todo_id = row.try_get("todo_id").map_err(ListError::from_err)?;
                let tag_entity = TagModel::from_row(&row)
                    .map_err(ListError::from_err)
                    .and_then(|model| model.try_into_entity().map_err(ListError::from_err))?;
                Ok((todo_id, tag_entity))
            })
            .collect::<Result<Vec<(Uuid, TagEntity)>, ListError>>()?;

        let mut todo_entities = todo_models
            .into_iter()
            .map(|m| m.try_into_entity(Vec::new()).map_err(ListError::from_err))
            .collect::<Result<Vec<TodoEntity>, ListError>>()?;

        for (todo_id, tag) in tag_relation_entries.into_iter() {
            for todo in todo_entities.iter_mut() {
                if todo.id.into_uuid() == todo_id {
                    todo.tags.push(tag);
                    break;
                }
            }
        }

        Ok(ListData {
            count: count as u64,
            items: todo_entities,
        })
    }
}

#[async_trait]
impl Update for TodoRepository {
    async fn update(&self, payload: UpdatePayload) -> Result<(), UpdateError> {
        let update_q = r#"
            UPDATE todo
            SET title = $1, description = $2, todo_at = $3, done = $4, updated_at = $5
            WHERE id = $6
        "#;

        sqlx::query(update_q)
            .bind(payload.title.into_string())
            .bind(payload.description.map(|d| d.into_string()))
            .bind(payload.todo_at.map(|at| at.into_date()))
            .bind(payload.done)
            .bind(payload.updated_at.into_date_time())
            .bind(payload.id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::from_err(err),
            })?;

        Ok(())
    }
}
