use async_trait::async_trait;
use sqlx::types::uuid::Uuid;
use sqlx::{Error as SqlxError, FromRow, PgPool, Postgres, QueryBuilder, Row};

use crate::application::repositories::todo::{
    BindTagsError, CreateError, DeleteError, ExistsError, FindError, ListError, ListPayload,
    PaginatedList, TodoRepository, UpdateError,
};

use crate::domain::entities::tag::TagEntity;
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::{DateTime, Id};
use crate::framework::storage::models::tag::TagModel;
use crate::framework::storage::models::todo::{TodoModel, TodoStatus as TodoModelStatus};

#[derive(Clone)]
pub struct PgTodoRepository {
    pool: PgPool,
}

impl PgTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PgTodoRepository {
    async fn find_related_tags(&self, todo_uuid: Uuid) -> Result<Vec<TagModel>, SqlxError> {
        let find_related_tags_q = r#"
            SELECT tag.*
            FROM tag
            INNER JOIN todo_tag ON todo_tag.tag_id = tag.id
            WHERE todo_tag.todo_id = $1
        "#;

        sqlx::query_as::<_, TagModel>(find_related_tags_q)
            .bind(todo_uuid)
            .fetch_all(&self.pool)
            .await
    }

    async fn find_many_related_tags(
        &self,
        todo_uuids: &[Uuid],
    ) -> Result<Vec<(Uuid, TagModel)>, SqlxError> {
        let find_many_related_tags_q = r#"
            SELECT todo_id, tag.*
            FROM todo_tag
            INNER JOIN tag ON tag.id = todo_tag.tag_id
            WHERE todo_tag.todo_id = ANY($1) 
        "#;

        let related_tags = sqlx::query(find_many_related_tags_q)
            .bind(todo_uuids)
            .fetch_all(&self.pool)
            .await?;

        related_tags
            .into_iter()
            .map(|row| Ok((row.try_get("todo_id")?, TagModel::from_row(&row)?)))
            .collect::<Result<Vec<(Uuid, TagModel)>, SqlxError>>()
    }
}

#[async_trait]
impl TodoRepository for PgTodoRepository {
    async fn bind_tags(&self, todo_id: Id, tag_ids: Vec<Id>) -> Result<(), BindTagsError> {
        let mut trx = self.pool.begin().await.map_err(BindTagsError::from_err)?;
        let todo_uuid = todo_id.into_uuid();

        let delete_relations_q = "DELETE FROM todo_tag WHERE todo_id = $1";
        sqlx::query(delete_relations_q)
            .bind(todo_uuid)
            .execute(trx.as_mut())
            .await
            .map_err(BindTagsError::from_err)?;

        let tag_uuids = tag_ids
            .into_iter()
            .map(|id| id.into_uuid())
            .collect::<Vec<Uuid>>();

        if !tag_uuids.is_empty() {
            let current_dt = DateTime::new().into_offset_dt();
            let base_bind_tags_q = "INSERT INTO todo_tag (todo_id, tag_id, created_at) ";
            QueryBuilder::<'_, Postgres>::new(base_bind_tags_q)
                .push_values(tag_uuids, |mut q, tag_id| {
                    q.push_bind(todo_uuid)
                        .push_bind(tag_id)
                        .push_bind(current_dt);
                })
                .build()
                .execute(trx.as_mut())
                .await
                .map_err(BindTagsError::from_err)?;
        }

        trx.commit().await.map_err(BindTagsError::from_err)
    }

    async fn create(&self, todo: TodoEntity) -> Result<TodoEntity, CreateError> {
        let current_dt = DateTime::new().into_offset_dt();
        let insert_q = r#"
            INSERT INTO todo (id, title, description, todo_at, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING todo.*
        "#;

        let todo_model = sqlx::query_as::<_, TodoModel>(insert_q)
            .bind(Id::new().into_uuid())
            .bind(todo.title.into_string())
            .bind(todo.description.map(|d| d.into_string()))
            .bind(todo.todo_at.map(|at| at.into_date()))
            .bind(TodoModelStatus::from(todo.status))
            .bind(current_dt)
            .bind(current_dt)
            .fetch_one(&self.pool)
            .await
            .map_err(CreateError::from_err)?;

        todo_model
            .try_into_entity(Vec::new())
            .map_err(CreateError::from_err)
    }

    async fn delete(&self, todo_id: Id) -> Result<(), DeleteError> {
        let delete_q = "DELETE FROM todo WHERE id = $1 RETURNING id";
        sqlx::query_scalar::<_, Uuid>(delete_q)
            .bind(todo_id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::from_err(err),
            })?;

        Ok(())
    }

    async fn exists(&self, todo_id: Id) -> Result<bool, ExistsError> {
        let todo_exists_q = "SELECT EXISTS(SELECT 1 FROM todo WHERE id = $1)";
        sqlx::query_scalar::<_, bool>(todo_exists_q)
            .bind(todo_id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(ExistsError::from_err)
    }

    async fn find(&self, todo_id: Id) -> Result<TodoEntity, FindError> {
        let todo_uuid = todo_id.into_uuid();
        let find_todo_q = "SELECT todo.* FROM todo WHERE id = $1";
        let todo_model = sqlx::query_as::<_, TodoModel>(find_todo_q)
            .bind(todo_uuid)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => FindError::from_err(err),
            })?;

        let related_tag_entities = self
            .find_related_tags(todo_uuid)
            .await
            .map_err(FindError::from_err)?
            .into_iter()
            .map(|model| model.try_into_entity().map_err(FindError::from_err))
            .collect::<Result<Vec<TagEntity>, FindError>>()?;

        todo_model
            .try_into_entity(related_tag_entities)
            .map_err(FindError::from_err)
    }

    async fn list(&self, payload: ListPayload) -> Result<PaginatedList, ListError> {
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

        let todo_uuids = todo_models
            .iter()
            .map(|todo| todo.id)
            .collect::<Vec<Uuid>>();

        let related_tag_models = self
            .find_many_related_tags(&todo_uuids)
            .await
            .map_err(ListError::from_err)?;

        let related_tag_entities = related_tag_models
            .into_iter()
            .map(|(todo_uuid, tag_model)| {
                let tag_entity = tag_model.try_into_entity().map_err(ListError::from_err)?;
                Ok((todo_uuid, tag_entity))
            })
            .collect::<Result<Vec<(Uuid, TagEntity)>, ListError>>()?;

        let mut todo_entities = todo_models
            .into_iter()
            .map(|m| m.try_into_entity(Vec::new()).map_err(ListError::from_err))
            .collect::<Result<Vec<TodoEntity>, ListError>>()?;

        for (todo_uuid, tag) in related_tag_entities.into_iter() {
            for todo in todo_entities.iter_mut() {
                if todo.id.into_uuid() == todo_uuid {
                    todo.tags.push(tag);
                    break;
                }
            }
        }

        Ok(PaginatedList {
            count: count as u64,
            items: todo_entities,
        })
    }

    async fn update(&self, todo: TodoEntity) -> Result<(), UpdateError> {
        let update_q = r#"
            UPDATE todo
            SET title = $1, description = $2, todo_at = $3, status = $4, updated_at = $5
            WHERE id = $6
        "#;

        sqlx::query(update_q)
            .bind(todo.title.into_string())
            .bind(todo.description.map(|d| d.into_string()))
            .bind(todo.todo_at.map(|at| at.into_date()))
            .bind(TodoModelStatus::from(todo.status))
            .bind(todo.updated_at.into_offset_dt())
            .bind(todo.id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::from_err(err),
            })?;

        Ok(())
    }
}
