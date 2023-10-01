use async_trait::async_trait;
use sqlx::{Error as SqlxError, Pool, Postgres};
use time::OffsetDateTime;

use super::models::todo::TodoModel;
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::application::repositories::todo::find::{Find, FindError};
use crate::application::repositories::todo::list::{List, ListError};
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
            SELECT * FROM todo 
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
        let q = r#"
            INSERT INTO todo
                (id, title, description, todo_at, created_at, updated_at)
            VALUES 
                ($1, $2, $3, $4, $5, $6)
        "#;

        let current_date_time = OffsetDateTime::now_utc();
        let todo_model = TodoModel {
            id: Id::new().into_uuid(),
            title: payload.title.into_string(),
            description: payload.description.into_opt_string(),
            todo_at: payload.todo_at.map(|at| at.into_date()),
            created_at: current_date_time,
            updated_at: current_date_time,
        };

        sqlx::query(q)
            .bind(&todo_model.id)
            .bind(&todo_model.title)
            .bind(&todo_model.description)
            .bind(&todo_model.todo_at)
            .bind(&todo_model.created_at)
            .bind(&todo_model.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("create todo repository error {err:?}");
                CreateError::Internal
            })?;

        todo_model_to_entity(todo_model).map_err(|_| CreateError::Internal)
    }
}

#[async_trait]
impl List for TodoStore {
    async fn list(&self) -> Result<Vec<TodoEntity>, ListError> {
        let q = r#"SELECT * FROM todo"#;

        let todo_models = sqlx::query_as::<_, TodoModel>(q)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| ListError::Internal)?;

        todo_models
            .into_iter()
            .map(todo_model_to_entity)
            .collect::<Result<Vec<TodoEntity>, ()>>()
            .map_err(|_| ListError::Internal)
    }
}

#[async_trait]
impl Delete for TodoStore {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let delete_q = r#"
            DELETE FROM todo 
            WHERE id = ($1)
        "#;

        sqlx::query(delete_q)
            .bind(id.into_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::Internal,
            })?;

        Ok(())
    }
}

#[async_trait]
impl Update for TodoStore {
    async fn update(&self, payload: UpdatePayload) -> Result<TodoEntity, UpdateError> {
        let q = r#"
            UPDATE todo
            SET title, description, todo_at
            VALUES ($1), ($2), ($3)
            WHERE id = ($4)
        "#;

        sqlx::query(q)
            .bind(payload.title.into_string())
            .bind(payload.description.into_opt_string())
            .bind(payload.todo_at.map(|at| at.into_date()))
            .bind(payload.id.into_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::Internal,
            })?;

        let todo = self.find(payload.id).await.map_err(|err| match err {
            FindError::NotFound => UpdateError::NotFound,
            FindError::Internal => UpdateError::Internal,
        })?;

        Ok(todo)
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
