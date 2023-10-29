use crate::domain::entities::todo::{Description, Title};
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug)]
pub struct UpdateTodoInput {
    pub id: Id,
    pub title: Title,
    pub description: Description,
    pub done: bool,
    pub todo_at: Option<Date>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateTodoError {
    NotFound,
    Internal,
}
