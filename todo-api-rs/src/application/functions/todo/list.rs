use crate::domain::todo::Todo;

pub trait TodoLister {
    fn list(&self) -> Result<Vec<Todo>, String>;
}

pub struct ListContext<T: TodoLister> {
    pub store: T,
}

pub async fn list_todo<T: TodoLister>(ctx: ListContext<T>) -> Result<Vec<Todo>, String> {
    ctx.store.list()
}
