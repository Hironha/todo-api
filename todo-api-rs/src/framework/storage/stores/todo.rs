use async_trait::async_trait;
use sqlx::types::Uuid;
use sqlx::{Error as SqlxError, Pool, Postgres};
use time::OffsetDateTime;

use super::models::todo::TodoModel;
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::application::repositories::todo::find::{Find, FindError};
use crate::application::repositories::todo::list::{List, ListData, ListError, ListPayload};
use crate::application::repositories::todo::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::domain::types::{Date, Id};

#[derive(Clone)]
pub struct TodoStore {
    pool: Pool<Postgres>,
}

impl TodoStore {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Find for TodoStore {
    async fn find(&self, id: Id) -> Result<TodoEntity, FindError> {
        let q = r#"
            SELECT id, title, description, todo_at, created_at, updated_at
            FROM todo 
            WHERE id = ($1)
        "#;

        let todo_model = sqlx::query_as::<_, TodoModel>(q)
            .bind(id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => {
                    tracing::error!("find todo repository error {err:?}");
                    FindError::Internal
                }
            })?;

        todo_model_to_entity(todo_model).map_err(|_| FindError::Internal)
    }
}

#[async_trait]
impl Create for TodoStore {
    async fn create(&self, payload: CreatePayload) -> Result<TodoEntity, CreateError> {
        let mut trx = self.pool.begin().await.map_err(|err| {
            tracing::error!("create todo failed beginning transaction {err:?}");
            CreateError::Internal
        })?;

        let count_q = r#" SELECT COUNT(count) FROM todo_count "#;
        let todo_count = sqlx::query_scalar::<_, i64>(count_q)
            .fetch_one(trx.as_mut())
            .await
            .map_err(|err| {
                tracing::error!("create todo failed counting {err:?}");
                CreateError::Internal
            })?;

        let update_count_q = match todo_count {
            0 => r#" INSERT INTO todo_count (count) VALUES (1) "#,
            1 => r#" UPDATE todo_count SET count = count + 1 "#,
            // `todo_count` table must either be empty or have one item
            _ => return Err(CreateError::Internal),
        };

        let update_count_result = sqlx::query(update_count_q)
            .execute(trx.as_mut())
            .await
            .map_err(|err| {
                tracing::error!("create todo failed updating count {err:?}");
                CreateError::Internal
            })?;

        if update_count_result.rows_affected() == 0 {
            return Err(CreateError::Internal);
        }

        let insert_q = r#"
            INSERT INTO todo (id, title, description, todo_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, description, todo_at, created_at, updated_at
        "#;

        let current_date_time = OffsetDateTime::now_utc();
        let model = sqlx::query_as::<_, TodoModel>(insert_q)
            .bind(Id::new().into_uuid())
            .bind(payload.title.into_string())
            .bind(payload.description.into_opt_string())
            .bind(payload.todo_at.map(|at| at.into_date()))
            .bind(current_date_time)
            .bind(current_date_time)
            .fetch_one(trx.as_mut())
            .await
            .map_err(|err| {
                tracing::error!("create todo failed creating {err:?}");
                CreateError::Internal
            })?;

        let todo_entity = todo_model_to_entity(model).map_err(|_| CreateError::Internal)?;

        trx.commit().await.map_err(|err| {
            tracing::error!("create todo failed committing transaction {err:?}");
            CreateError::Internal
        })?;

        Ok(todo_entity)
    }
}

#[async_trait]
impl List for TodoStore {
    async fn list(&self, payload: ListPayload) -> Result<ListData, ListError> {
        let limit: i64 = u32::from(payload.per_page).into();
        let page: i64 = u32::from(payload.page).into();
        let offset = (page - 1) * limit;
        let list_q = r#"
            SELECT id, title, description, todo_at, created_at, updated_at
            FROM todo
            ORDER BY created_at DESC
            LIMIT ($1)
            OFFSET ($2)
        "#;

        let todo_models = sqlx::query_as::<_, TodoModel>(list_q)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("list todo failed {err:?}");
                ListError::Internal
            })?;

        // count `todo` by querying from `todo_count` table
        let count_q = r#" SELECT count FROM todo_count LIMIT 1 "#;
        let count_result = sqlx::query_scalar::<_, i64>(count_q)
            .fetch_one(&self.pool)
            .await;

        let db_count = match count_result {
            Ok(count) => count,
            Err(SqlxError::RowNotFound) => 0,
            Err(err) => {
                tracing::error!("list todo failed counting {err:?}");
                return Err(ListError::Internal);
            }
        };

        let count: u64 = db_count.try_into().map_err(|err| {
            tracing::error!("list todo with invalid count {err:?}");
            ListError::Internal
        })?;

        let todo_entities = todo_models
            .into_iter()
            .map(todo_model_to_entity)
            .collect::<Result<Vec<TodoEntity>, ()>>()
            .map_err(|_| ListError::Internal)?;

        Ok(ListData {
            count,
            items: todo_entities,
        })
    }
}

#[async_trait]
impl Delete for TodoStore {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let mut trx = self.pool.begin().await.map_err(|err| {
            tracing::error!("delete todo failed beginning transaction {err:?}");
            DeleteError::Internal
        })?;

        let delete_q = r#" DELETE FROM todo WHERE id = ($1) RETURNING id "#;
        sqlx::query_scalar::<_, Uuid>(delete_q)
            .bind(id.into_uuid())
            .fetch_one(trx.as_mut())
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => {
                    tracing::error!("delete todo failed deleting {err:?}");
                    DeleteError::Internal
                }
            })?;

        let count_q = r#" SELECT COUNT(count) FROM todo_count "#;
        let count = sqlx::query_scalar::<_, i64>(count_q)
            .fetch_one(trx.as_mut())
            .await
            .map_err(|err| {
                tracing::error!("delete todo failed counting {err:?}");
                DeleteError::Internal
            })?;

        // update `todo_count` only if already exists an item
        if count > 0 {
            let decrement_count_q = r#" UPDATE todo_count SET count = count - 1 "#;
            sqlx::query(decrement_count_q)
                .execute(trx.as_mut())
                .await
                .map_err(|err| {
                    tracing::error!("delete todo failed updating count {err:?}");
                    DeleteError::Internal
                })?;
        }

        trx.commit().await.map_err(|err| {
            tracing::error!("delete todo failed committing transaction {err:?}");
            DeleteError::Internal
        })
    }
}

#[async_trait]
impl Update for TodoStore {
    async fn update(&self, payload: UpdatePayload) -> Result<TodoEntity, UpdateError> {
        let q = r#"
            UPDATE todo
            SET title = ($1), description = ($2), todo_at = ($3)
            WHERE id = ($4)
            RETURNING id, title, description, todo_at, created_at, updated_at
        "#;

        let model = sqlx::query_as::<_, TodoModel>(q)
            .bind(payload.title.into_string())
            .bind(payload.description.into_opt_string())
            .bind(payload.todo_at.map(|at| at.into_date()))
            .bind(payload.id.into_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => {
                    tracing::error!("update todo repository error: {err:?}");
                    UpdateError::Internal
                }
            })?;

        todo_model_to_entity(model).map_err(|_| UpdateError::Internal)
    }
}

fn todo_model_to_entity(model: TodoModel) -> Result<TodoEntity, ()> {
    let title = Title::new(model.title).map_err(|e| {
        let msg = e.description();
        tracing::error!("todo model title is incompatible with entity title: {msg}");
    })?;
    let description = Description::new(model.description).map_err(|err| {
        let msg = err.description();
        tracing::error!("todo model description is incompatible with entity description: {msg}");
    })?;

    Ok(TodoEntity {
        id: model.id.into(),
        title,
        description,
        todo_at: model.todo_at.map(Date::from),
        created_at: model.created_at.into(),
        updated_at: model.updated_at.into(),
    })
}
