use super::types::{Date, DateTime, Id};

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    pub todo_at: Option<Date>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
