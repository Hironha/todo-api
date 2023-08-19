use async_trait::async_trait;
use sqlx::{types::Uuid, Pool, Postgres};

use super::models::todo::TodoModel;
use crate::{
    application::functions::todo::{
        Create, CreatePayload, Delete, Find, List, Update, UpdatePayload,
    },
    domain::todo::Todo,
};

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
    async fn find(&self, id: &Uuid) -> Result<Todo, String> {
        let q = r"SELECT * FROM todos WHERE id = ($1)";

        let model = sqlx::query_as::<_, TodoModel>(q)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                println!("{err:?}");
                format!("failed to find todo with id {id}")
            })?;

        Ok(model.into_entity())
    }
}

#[async_trait]
impl Create for TodoStore {
    async fn create(&self, payload: CreatePayload) -> Result<Todo, String> {
        let q = r"
            INSERT INTO todos (id, title, description, todo_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        ";

        let model = TodoModel::from_payload(payload);
        sqlx::query(q)
            .bind(model.id)
            .bind(&model.title)
            .bind(&model.description)
            .bind(model.todo_at)
            .bind(model.created_at)
            .bind(model.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|_| "failed to create todo".to_string())?;

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
    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let delete_q = r"DELETE FROM todos WHERE id = ($1)";

        sqlx::query(delete_q)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_| format!("failed to delete todo with id {id}"))?;

        Ok(())
    }
}

#[async_trait]
impl Update for TodoStore {
    async fn set(&self, payload: UpdatePayload) -> Result<Todo, String> {
        let q = r"
            UPDATE todos 
            SET title, description, todo_at
            VALUES ($1), ($2), ($3)
            WHERE id = ($4)
        ";

        sqlx::query(q)
            .bind(payload.title)
            .bind(payload.description)
            .bind(payload.todo_at.map(|at| at.to_date()))
            .bind(payload.id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                println!("{err:?}");
                "failed to update todo".to_string()
            })?;

        let todo = self.find(&payload.id).await?;

        Ok(todo)
    }
}
