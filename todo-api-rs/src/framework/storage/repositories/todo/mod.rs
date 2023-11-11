mod bind_tags;
mod count;
mod create;
mod delete;
mod find;
mod list;
mod update;

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::application::repositories::todo::bind_tags::{BindTags, BindTagsError, BindTagsPayload};
use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::application::repositories::todo::delete::{Delete, DeleteError};
use crate::application::repositories::todo::find::{Find, FindError};
use crate::application::repositories::todo::list::{List, ListData, ListError, ListPayload};
use crate::application::repositories::todo::update::{Update, UpdateError, UpdatePayload};
use crate::domain::entities::todo::{Description, DescriptionError, Title, TitleError, TodoEntity};
use crate::domain::types::{Date, Id};
use crate::framework::storage::models::todo::TodoModel;

use bind_tags::bind_tags;
use count::{count_todo, CountTodoFilters};
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
impl BindTags for TodoRepository {
    async fn bind_tags(&self, payload: BindTagsPayload) -> Result<(), BindTagsError> {
        let mut trx = self.pool.begin().await.map_err(BindTagsError::from_err)?;
        bind_tags(&mut trx, payload).await?;
        trx.commit().await.map_err(BindTagsError::from_err)
    }
}

#[async_trait]
impl Create for TodoRepository {
    async fn create(&self, payload: CreatePayload) -> Result<TodoEntity, CreateError> {
        let mut conn = self.pool.acquire().await.map_err(CreateError::from_err)?;
        let model = create_todo(conn.as_mut(), payload).await?;
        map_todo_model_to_entity(model).map_err(CreateError::from_err)
    }
}

#[async_trait]
impl Delete for TodoRepository {
    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        let mut conn = self.pool.acquire().await.map_err(DeleteError::from_err)?;
        delete_todo(conn.as_mut(), id).await
    }
}

#[async_trait]
impl Find for TodoRepository {
    async fn find(&self, id: Id) -> Result<TodoEntity, FindError> {
        let mut conn = self.pool.acquire().await.or(Err(FindError::Internal))?;
        let model = find_todo(conn.as_mut(), id).await?;
        map_todo_model_to_entity(model).or(Err(FindError::Internal))
    }
}

#[async_trait]
impl List for TodoRepository {
    async fn list(&self, payload: ListPayload) -> Result<ListData, ListError> {
        let mut conn = self.pool.acquire().await.or(Err(ListError::Internal))?;
        let count_filters = CountTodoFilters {
            title: payload.title.as_ref().map(|t| t.as_str()),
        };

        let db_count = count_todo(conn.as_mut(), count_filters)
            .await
            .map_err(|err| {
                tracing::error!("count todo error: {err:?}");
                ListError::Internal
            })?;

        let count = u64::try_from(db_count).or(Err(ListError::Internal))?;

        let todo_models = list_todo(conn.as_mut(), payload).await?;
        let todo_entities = todo_models
            .into_iter()
            .map(map_todo_model_to_entity)
            .collect::<Result<Vec<TodoEntity>, MapTodoModelError>>()
            .map_err(|_| ListError::Internal)?;

        Ok(ListData {
            count,
            items: todo_entities,
        })
    }
}

#[async_trait]
impl Update for TodoRepository {
    async fn update(&self, payload: UpdatePayload) -> Result<TodoEntity, UpdateError> {
        let todo = update_todo(&self.pool, payload).await?;
        map_todo_model_to_entity(todo).or(Err(UpdateError::Internal))
    }
}

fn map_todo_model_to_entity(model: TodoModel) -> Result<TodoEntity, MapTodoModelError> {
    let title = Title::new(model.title).map_err(MapTodoModelError::Title)?;
    let description =
        Description::new(model.description).map_err(MapTodoModelError::Description)?;

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

#[derive(Debug)]
enum MapTodoModelError {
    Title(TitleError),
    Description(DescriptionError),
}

impl fmt::Display for MapTodoModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo model incompatible with entity")
    }
}

impl Error for MapTodoModelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Title(err) => Some(err),
            Self::Description(err) => Some(err),
        }
    }
}
