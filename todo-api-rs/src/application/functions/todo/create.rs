use crate::application::repositories::todo::create::{Create, CreateError, CreatePayload};
use crate::domain::todo::{Description, DescriptionError, Title, TitleError, Todo};
use crate::domain::types::Date;

pub async fn create_todo<T: Create>(
    ctx: CreateContext<T>,
    input: CreateTodoInput,
) -> Result<Todo, CreateTodoError> {
    let title = Title::new(input.title).map_err(CreateTodoError::Title)?;
    let description = Description::new(input.description).map_err(CreateTodoError::Description)?;
    let payload = CreatePayload {
        title,
        description,
        todo_at: input.todo_at,
    };

    ctx.store.create(payload).await.map_err(|e| match e {
        CreateError::Internal => CreateTodoError::Internal,
    })
}

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
}

pub struct CreateContext<T: Create> {
    pub store: T,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CreateTodoError {
    Title(TitleError),
    Description(DescriptionError),
    Internal,
}
