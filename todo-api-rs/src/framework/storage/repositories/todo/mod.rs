mod create;
mod delete;
mod find;
mod list;
mod update;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::application::repositories::todo::find::{Find, FindError};
use crate::application::repositories::todo::list::{List, ListData, ListError, ListPayload};
use crate::application::repositories::todo::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::todo::{Description, Title, TodoEntity};
use crate::domain::types::{Date, Id};
use crate::framework::storage::models::todo::TodoModel;

use create::create_todo;
use delete::delete_todo;
use find::find_todo;
use list::list_todo;
use update::update_todo;

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
impl Create for TodoRepository {
    async fn create(&self, payload: CreatePayload) -> Result<TodoEntity, CreateError> {
        let model = create_todo(&self.pool, payload).await?;
        map_todo_model_to_entity(model).map_err(|_| CreateError::Internal)
    }
}

#[async_trait]
impl Find for TodoRepository {
    async fn find(&self, id: Id) -> Result<TodoEntity, FindError> {
        let model = find_todo(&self.pool, id).await?;
        map_todo_model_to_entity(model).map_err(|_| FindError::Internal)
    }
}

#[async_trait]
impl List for TodoRepository {
    async fn list(&self, payload: ListPayload) -> Result<ListData, ListError> {
        let data = list_todo(&self.pool, payload).await?;
        let todo_entities = data
            .items
            .into_iter()
            .map(map_todo_model_to_entity)
            .collect::<Result<Vec<TodoEntity>, ()>>()
            .map_err(|_| ListError::Internal)?;

        Ok(ListData {
            count: data.count,
            items: todo_entities,
        })
    }
}

#[async_trait]
impl Delete for TodoRepository {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        delete_todo(&self.pool, id).await
    }
}

#[async_trait]
impl Update for TodoRepository {
    async fn update(&self, payload: UpdatePayload) -> Result<TodoEntity, UpdateError> {
        let todo = update_todo(&self.pool, payload).await?;
        map_todo_model_to_entity(todo).map_err(|_| UpdateError::Internal)
    }
}

fn map_todo_model_to_entity(model: TodoModel) -> Result<TodoEntity, ()> {
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
        done: model.done,
        todo_at: model.todo_at.map(Date::from),
        created_at: model.created_at.into(),
        updated_at: model.updated_at.into(),
    })
}
