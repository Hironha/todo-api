use std::error::Error;

use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::{Error as SqlxError, PgPool, Postgres, QueryBuilder};

use crate::application::repositories::todo::{
    CreateError, DeleteError, FindError, ListError, ListQuery, PaginatedList, TodoRepository,
    UpdateError, UpdateQuery,
};

use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::{DateTime, Id};
use crate::framework::storage::models::todo::{Status as TodoModelStatus, TodoModel};

#[derive(Clone)]
pub struct PgTodoRepository {
    pool: PgPool,
}

impl PgTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TodoRepository for PgTodoRepository {
    async fn create(&mut self, todo: TodoEntity) -> Result<(), CreateError> {
        const INSERT_Q: &str = r#"
            INSERT INTO todo (id, title, description, todo_at, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#;

        let now = DateTime::now();
        sqlx::query(INSERT_Q)
            .bind(todo.id().uuid())
            .bind(todo.title.as_str())
            .bind(todo.description.as_ref().map(|d| d.as_str()))
            .bind(todo.todo_at.map(|at| at.time()))
            .bind(TodoModelStatus::from(&todo.status))
            .bind(todo.created_at().unwrap_or(now).time())
            .bind(todo.updated_at().unwrap_or(now).time())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                    CreateError::DuplicatedTitle
                }
                _ => CreateError::Internal(err.into()),
            })?;

        Ok(())
    }

    async fn delete(&mut self, todo_id: Id) -> Result<(), DeleteError> {
        const DELETE_Q: &str = "DELETE FROM todo WHERE id = $1 RETURNING id";
        sqlx::query_scalar::<_, Uuid>(DELETE_Q)
            .bind(todo_id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::Internal(err.into()),
            })?;

        Ok(())
    }

    async fn find(&self, todo_id: Id) -> Result<TodoEntity, FindError> {
        const FIND_Q: &str = r#" SELECT * FROM todo as t WHERE t.id = $1"#;

        let model = sqlx::query_as::<_, TodoModel>(FIND_Q)
            .bind(todo_id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => FindError::Internal(err.into()),
            })?;

        model.try_into_entity().map_err(FindError::Internal)
    }

    async fn list(&self, query: ListQuery) -> Result<PaginatedList, ListError> {
        let mut count_q = QueryBuilder::<Postgres>::new(r#" SELECT COUNT(*) FROM todo as t "#);
        let mut list_q = QueryBuilder::<Postgres>::new(r#" SELECT * FROM todo "#);

        let title_filter = query.title.as_ref().map(|t| format!("%{}%", t.as_str()));
        if let Some(constraint) = title_filter.as_deref() {
            count_q.push(" WHERE title ILIKE ").push_bind(constraint);
            list_q.push(" WHERE title ILIKE ").push_bind(constraint);
        }

        let count = count_q
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ListError::Internal(e.into()))?;

        let limit: i64 = u32::from(query.per_page).into();
        let page: i64 = u32::from(query.page).into();
        let offset = (page - 1) * limit;

        let models = list_q
            .push(" ORDER BY created_at DESC LIMIT ")
            .push_bind(limit)
            .push(" OFFSET ")
            .push_bind(offset)
            .build_query_as::<TodoModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ListError::Internal(err.into()))?;

        let entities = models
            .into_iter()
            .map(TodoModel::try_into_entity)
            .collect::<Result<Vec<TodoEntity>, Box<dyn Error>>>()
            .map_err(ListError::Internal)?;

        Ok(PaginatedList {
            count: count as u64,
            items: entities,
        })
    }

    async fn update(&mut self, query: UpdateQuery) -> Result<(), UpdateError> {
        const UPDATE_Q: &str = r#"
            UPDATE todo
            SET title = $1, description = $2, todo_at = $3, status = $4, updated_at = $5
            WHERE id = $6
        "#;

        sqlx::query(UPDATE_Q)
            .bind(query.title.into_inner())
            .bind(query.description.map(|d| d.into_inner()))
            .bind(query.todo_at.map(|at| at.time()))
            .bind(TodoModelStatus::from(query.status))
            .bind(OffsetDateTime::now_utc())
            .bind(query.id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                    UpdateError::DuplicatedTitle
                }
                SqlxError::RowNotFound => UpdateError::NotFound,
                _ => UpdateError::Internal(err.into()),
            })?;

        Ok(())
    }
}
