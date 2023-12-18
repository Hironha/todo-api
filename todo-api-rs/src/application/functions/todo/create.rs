use crate::application::dtos::todo::create::{CreateTodoError, CreateTodoInput};
use crate::application::repositories::todo::TodoRepository;
use crate::domain::entities::todo::TodoEntity;
use crate::domain::types::{DateTime, Id};

pub async fn create_todo<T>(
    ctx: CreateTodoContext<'_, T>,
    input: CreateTodoInput,
) -> Result<TodoEntity, CreateTodoError>
where
    T: TodoRepository,
{
    let current_dt = DateTime::new();
    let todo_entity = TodoEntity {
        id: Id::new(),
        title: input.title,
        description: input.description,
        status: input.status,
        todo_at: input.todo_at,
        tags: Vec::new(),
        created_at: current_dt,
        updated_at: current_dt,
    };

    ctx.todo_repository
        .create(todo_entity)
        .await
        .map_err(CreateTodoError::Creating)
}

#[derive(Clone, Debug)]
pub struct CreateTodoContext<'a, T>
where
    T: TodoRepository,
{
    pub todo_repository: &'a T,
}
