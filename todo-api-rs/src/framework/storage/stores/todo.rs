use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::models::todo::TodoModel;
use crate::{
    application::functions::todo::{
        CreatePayload, TodoCreator, TodoDeleter, TodoGetter, TodoLister, TodoSetter, UpdatePayload,
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
impl TodoGetter for TodoStore {
    async fn get(&self, id: &str) -> Result<Todo, String> {
        let q = r"SELECT * FROM todos WHERE todos.id = ($1)";

        let model = sqlx::query_as::<_, TodoModel>(q)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| format!("failed to find todo with id {id}"))?;

        Ok(model.into())
    }
}

#[async_trait]
impl TodoCreator for TodoStore {
    async fn create(&self, payload: CreatePayload) -> Result<Todo, String> {
        let q = r"
            INSERT INTO todos (id, title, description, todo_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        ";

        let model = TodoModel::from(payload);

        sqlx::query(q)
            .bind(&model.id)
            .bind(&model.title)
            .bind(&model.description)
            .bind(&model.todo_at)
            .bind(&model.created_at)
            .bind(&model.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|_| "failed to create todo".to_string())?;

        Ok(model.into())
    }
}

#[async_trait]
impl TodoLister for TodoStore {
    async fn list(&self) -> Result<Vec<Todo>, String> {
        let q = "SELECT * FROM todos";

        let res = sqlx::query_as::<_, TodoModel>(q)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| "failed to list todos".to_string())?;

        let todos = res.into_iter().map(|r| r.into()).collect::<Vec<Todo>>();

        Ok(todos)
    }
}

#[async_trait]
impl TodoDeleter for TodoStore {
    async fn delete(&self, id: &str) -> Result<Todo, String> {
        let todo = self.get(id).await?;

        let delete_q = r"DELETE FROM todos WHERE todos.id = ($1)";
        sqlx::query(delete_q)
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();

        Ok(todo)
    }
}

#[async_trait]
impl TodoSetter for TodoStore {
    async fn set(&self, payload: UpdatePayload) -> Result<Todo, String> {
        let q = r"
            UPDATE todos SET title = ($1), description = ($2), todo_at = ($3)
            WHERE todos.id = ($4) 
        ";

        sqlx::query(q)
            .bind(&payload.title)
            .bind(&payload.description)
            .bind(&payload.todo_at)
            .bind(&payload.id)
            .execute(&self.pool)
            .await
            .map_err(|_| "failed to update todo".to_string())?;

        let todo = self.get(&payload.id).await?;

        Ok(todo)
    }
}
