use std::error::Error;

use sqlx::types::uuid::Uuid;
use sqlx::{Error as SqlxError, PgPool, Postgres, QueryBuilder};

use crate::application::repositories::todo::{
    BindTagsError, CreateError, DeleteError, ExistsError, ExistsTagsError, FindError, ListError,
    ListQuery, PaginatedList, TodoRepository, UpdateError,
};

use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::{DateTime, Id};
use crate::framework::storage::models::todo::{TodoModel, TodoStatus as TodoModelStatus};
use crate::framework::storage::views::TodoWithTagsView;

#[derive(Clone)]
pub struct PgTodoRepository {
    pool: PgPool,
}

impl PgTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn todo_with_tags_cte_query(&self) -> &'static str {
        r#"
        WITH todo_with_tags as (
            SELECT
                todo.id,
                todo.title,
                todo.description,
                todo.status,
                todo.todo_at,
                todo.created_at,
                todo.updated_at,
                CASE
                    WHEN COUNT(tag.id) > 0 THEN
                        jsonb_agg(jsonb_build_object(
                            'id', tag.id,
                            'name', tag.name,
                            'description', tag.description,
                            'created_at', tag.created_at,
                            'updated_at', tag.updated_at
                        ))
                    ELSE
                        '[]'::jsonb
                END as tags
            FROM todo
            LEFT JOIN todo_tag ON todo.id = todo_tag.todo_id
            LEFT JOIN tag ON todo_tag.tag_id = tag.id
            GROUP BY todo.id
        )
        "#
    }
}

impl TodoRepository for PgTodoRepository {
    async fn bind_tags(&mut self, todo_id: Id, tag_ids: &[Id]) -> Result<(), BindTagsError> {
        let mut trx = match self.pool.begin().await {
            Ok(trx) => trx,
            Err(err) => return Err(BindTagsError::Internal(err.into())),
        };

        let delete_relations_q = "DELETE FROM todo_tag WHERE todo_id = $1";
        sqlx::query(delete_relations_q)
            .bind(todo_id.uuid())
            .execute(trx.as_mut())
            .await
            .map_err(|err| BindTagsError::Internal(err.into()))?;

        let tag_uuids = tag_ids.iter().map(|id| id.uuid()).collect::<Vec<Uuid>>();

        if !tag_uuids.is_empty() {
            let current_dt = DateTime::now().date_time();
            let base_bind_tags_q = "INSERT INTO todo_tag (todo_id, tag_id, created_at) ";
            QueryBuilder::<Postgres>::new(base_bind_tags_q)
                .push_values(tag_uuids, |mut q, tag_id| {
                    q.push_bind(todo_id.uuid())
                        .push_bind(tag_id)
                        .push_bind(current_dt);
                })
                .build()
                .execute(trx.as_mut())
                .await
                .map_err(|err| BindTagsError::Internal(err.into()))?;
        }

        trx.commit()
            .await
            .map_err(|err| BindTagsError::Internal(err.into()))
    }

    async fn create(&mut self, todo: TodoEntity) -> Result<TodoEntity, CreateError> {
        let insert_q = r#"
            INSERT INTO todo (id, title, description, todo_at, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING todo.*
        "#;

        let todo_model = sqlx::query_as::<_, TodoModel>(insert_q)
            .bind(todo.id.uuid())
            .bind(todo.title.into_inner())
            .bind(todo.description.map(|d| d.into_inner()))
            .bind(todo.todo_at.map(|at| at.date()))
            .bind(TodoModelStatus::from(todo.status))
            .bind(todo.created_at.date_time())
            .bind(todo.updated_at.date_time())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                    CreateError::DuplicatedTitle
                }
                _ => CreateError::Internal(err.into()),
            })?;

        todo_model
            .try_into_entity(Vec::new())
            .map_err(|e| CreateError::Internal(e.into()))
    }

    async fn delete(&mut self, todo_id: Id) -> Result<(), DeleteError> {
        let delete_q = "DELETE FROM todo WHERE id = $1 RETURNING id";
        sqlx::query_scalar::<_, Uuid>(delete_q)
            .bind(todo_id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => DeleteError::NotFound,
                _ => DeleteError::Internal(err.into()),
            })?;

        Ok(())
    }

    async fn exists(&mut self, todo_id: Id) -> Result<bool, ExistsError> {
        let todo_exists_q = "SELECT EXISTS(SELECT 1 FROM todo WHERE id = $1)";
        sqlx::query_scalar::<_, bool>(todo_exists_q)
            .bind(todo_id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ExistsError::Internal(e.into()))
    }

    async fn find(&mut self, todo_id: Id) -> Result<TodoEntity, FindError> {
        let mut find_q = String::from(self.todo_with_tags_cte_query());
        find_q.push_str(r#" SELECT * FROM todo_with_tags as t WHERE t.id = $1"#);

        let todo_with_tags = sqlx::query_as::<_, TodoWithTagsView>(find_q.as_str())
            .bind(todo_id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => FindError::NotFound,
                _ => FindError::Internal(err.into()),
            })?;

        todo_with_tags
            .try_into_entity()
            .map_err(FindError::Internal)
    }

    async fn list(&mut self, query: ListQuery) -> Result<PaginatedList, ListError> {
        let base_list_q = self.todo_with_tags_cte_query();
        let mut count_q = QueryBuilder::<Postgres>::new(base_list_q);
        count_q.push(r#" SELECT COUNT(*) FROM todo_with_tags as t "#);

        let mut list_q = QueryBuilder::<Postgres>::new(base_list_q);
        list_q.push(r#" SELECT * FROM todo_with_tags "#);

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

        let todos_with_tags = list_q
            .push(" ORDER BY created_at DESC LIMIT ")
            .push_bind(limit)
            .push(" OFFSET ")
            .push_bind(offset)
            .build_query_as::<TodoWithTagsView>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ListError::Internal(e.into()))?;

        let todo_entities = todos_with_tags
            .into_iter()
            .map(TodoWithTagsView::try_into_entity)
            .collect::<Result<Vec<TodoEntity>, Box<dyn Error>>>()
            .map_err(ListError::Internal)?;

        Ok(PaginatedList {
            count: count as u64,
            items: todo_entities,
        })
    }

    async fn update(&mut self, todo: TodoEntity) -> Result<(), UpdateError> {
        let update_q = r#"
            UPDATE todo
            SET title = $1, description = $2, todo_at = $3, status = $4, updated_at = $5
            WHERE id = $6
        "#;

        sqlx::query(update_q)
            .bind(todo.title.into_inner())
            .bind(todo.description.map(|d| d.into_inner()))
            .bind(todo.todo_at.map(|at| at.date()))
            .bind(TodoModelStatus::from(todo.status))
            .bind(todo.updated_at.date_time())
            .bind(todo.id.uuid())
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

    async fn exists_tags(&mut self, tag_ids: &[Id]) -> Result<(), ExistsTagsError> {
        let tag_uuids = tag_ids.iter().map(|id| id.uuid()).collect::<Vec<Uuid>>();

        let select_any_q = "SELECT id FROM tag WHERE id = ANY($1)";
        let selected_tag_uuids = sqlx::query_scalar::<_, Uuid>(select_any_q)
            .bind(&tag_uuids)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ExistsTagsError::Internal(err.into()))?;

        let not_found_ids = tag_uuids
            .into_iter()
            .filter(|uuid| !selected_tag_uuids.contains(uuid))
            .map(Id::from)
            .collect::<Vec<Id>>();

        if not_found_ids.is_empty() {
            Ok(())
        } else {
            Err(ExistsTagsError::NotFound(not_found_ids))
        }
    }
}
