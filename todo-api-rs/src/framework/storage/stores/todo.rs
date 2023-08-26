use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::models::todo::TodoModel;
use crate::application::functions::todo::{
    Create, CreateError, CreatePayload, Delete, DeleteError, Find, FindError, List, Update,
    UpdateError, UpdatePayload,
};
use crate::domain::{todo::Todo, types::Id};

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
    async fn find(&self, id: &Id) -> Result<Todo, FindError> {
        let q = r"SELECT * FROM todos WHERE id = ($1)";

        let res = sqlx::query_as::<_, TodoModel>(q)
            .bind(id.uuid())
            .fetch_one(&self.pool)
            .await;

        res.map(|m| m.into_entity()).map_err(|err| {
            println!("FIND ERROR -> {err:?}");
            match err {
                sqlx::Error::RowNotFound => FindError::NotFound,
                _ => FindError::InternalError,
            }
        })
    }
}

#[async_trait]
impl Create for TodoStore {
    async fn create(&self, payload: CreatePayload) -> Result<Todo, CreateError> {
        let q = r"
            INSERT INTO todos (id, title, description, todo_at, created_at, updated_at)
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

        if let Err(err) = res {
            println!("CREATE ERROR -> {err:?}");
            return Err(CreateError::InternalError);
        }

        Ok(model.into_entity())
    }
}

#[async_trait]
impl List for TodoStore {
    async fn list(&self) -> Result<Vec<Todo>, String> {
        let q = r"SELECT * FROM todos";

        let res = sqlx::query_as::<_, TodoModel>(q)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                println!("{err:?}");
                "failed to list todos".to_string()
            })?;

        let todos = res
            .into_iter()
            .map(|model| model.into_entity())
            .collect::<Vec<Todo>>();

        Ok(todos)
    }
}

#[async_trait]
impl Delete for TodoStore {
    async fn delete(&self, id: &Id) -> Result<(), DeleteError> {
        let delete_q = r"DELETE FROM todos WHERE id = ($1)";

        let res = sqlx::query(delete_q)
            .bind(id.uuid())
            .execute(&self.pool)
            .await;

        if let Err(err) = res {
            println!("STORAGE -> DELETE TODO ERROR: {err:?}");
            let error = match err {
                sqlx::Error::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::InternalError,
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
            UPDATE todos 
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
            println!("UPDATE ERROR => {err:?}");
            let error = match err {
                sqlx::Error::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::InternalError,
            };
            return Err(error);
        }

        let todo = self.find(&payload.id).await.map_err(|err| match err {
            FindError::NotFound => UpdateError::NotFound,
            FindError::InternalError => UpdateError::InternalError,
        })?;

        Ok(todo)
    }
}
