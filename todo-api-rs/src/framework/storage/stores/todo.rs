use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::models::todo::TodoModel;
use crate::application::functions::todo::{Update, UpdateError, UpdatePayload};
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::application::repositories::todo::find::{Find, FindError};
use crate::application::repositories::todo::list::{List, ListError};
use crate::domain::todo::Todo;
use crate::domain::types::Id;

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
    async fn find(&self, id: Id) -> Result<Todo, FindError> {
        let q = r"SELECT * FROM Todo WHERE id = ($1)";

        let res = sqlx::query_as::<_, TodoModel>(q)
            .bind(id.uuid())
            .fetch_one(&self.pool)
            .await;

        res.map(|m| m.into_entity()).map_err(|err| match err {
            sqlx::Error::RowNotFound => FindError::NotFound,
            _ => FindError::Internal,
        })
    }
}

#[async_trait]
impl Create for TodoStore {
    async fn create(&self, payload: CreatePayload) -> Result<Todo, CreateError> {
        let q = r"
            INSERT INTO Todo (id, title, description, todo_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        ";

        let model = TodoModel::from(payload);
        let res = sqlx::query(q)
            .bind(model.id)
            .bind(&model.title)
            .bind(&model.description)
            .bind(model.todo_at)
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&self.pool)
            .await;

        if res.is_err() {
            return Err(CreateError::Internal);
        }

        Ok(model.into_entity())
    }
}

#[async_trait]
impl List for TodoStore {
    async fn list(&self) -> Result<Vec<Todo>, ListError> {
        let q = r"SELECT * FROM Todo";

        let res = sqlx::query_as::<_, TodoModel>(q)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| ListError::Internal)?;

        let todos = res
            .into_iter()
            .map(|model| model.into_entity())
            .collect::<Vec<Todo>>();

        Ok(todos)
    }
}

#[async_trait]
impl Delete for TodoStore {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let delete_q = r"DELETE FROM Todo WHERE id = ($1)";

        let res = sqlx::query(delete_q)
            .bind(id.uuid())
            .execute(&self.pool)
            .await;

        if let Err(err) = res {
            let error = match err {
                sqlx::Error::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::Internal,
            };
            return Err(error);
        }

        Ok(())
    }
}

#[async_trait]
impl Update for TodoStore {
    async fn set(&self, payload: UpdatePayload) -> Result<Todo, UpdateError> {
        let q = r"
            UPDATE Todo 
            SET title, description, todo_at
            VALUES ($1), ($2), ($3)
            WHERE id = ($4)
        ";

        let res = sqlx::query(q)
            .bind(payload.title)
            .bind(payload.description)
            .bind(payload.todo_at.map(|at| at.date()))
            .bind(payload.id.uuid())
            .execute(&self.pool)
            .await;

        if let Err(err) = res {
            let error = match err {
                sqlx::Error::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::InternalError,
            };
            return Err(error);
        }

        let todo = self.find(payload.id).await.map_err(|err| match err {
            FindError::NotFound => UpdateError::NotFound,
            FindError::Internal => UpdateError::InternalError,
        })?;

        Ok(todo)
    }
}
