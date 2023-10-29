use crate::domain::entities::todo::{Description, Title};
use crate::domain::types::Date;

#[derive(Clone, Debug)]
pub struct CreateTodoInput {
    pub title: Title,
    pub description: Description,
    pub todo_at: Option<Date>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTodoError {
    Internal,
}
