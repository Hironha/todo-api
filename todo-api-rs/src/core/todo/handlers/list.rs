use crate::core::todo::models::Todo;

pub trait TodoLister {
    fn list(&self) -> Result<Vec<Todo>, String>;
}

pub struct ListContext<T: TodoLister> {
    pub lister: T,
}

pub async fn list_todos<T: TodoLister>(ctx: ListContext<T>) -> Vec<Todo> {
    let result = ctx.lister.list();
    match result {
        Ok(todos) => todos,
        Err(err) => {
            println!("Something went wrong: {err}");
            panic!("Failed to list todos");
        }
    }
}
