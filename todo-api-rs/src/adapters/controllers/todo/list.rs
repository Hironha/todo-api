use crate::adapters::dtos::todo::list::{Output, OutputData, RunError};
use crate::adapters::views::todo::TodoView;
use crate::application::functions::todo::{list_todo, List, ListContext, ListError};

pub struct ListController<S: List> {
    store: S,
}

impl<S: List> ListController<S> {
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn run(self) -> Output {
        let context = ListContext { store: self.store };
        let result = list_todo(context).await.map_err(|e| match e {
            ListError::StorageAccess => RunError::Internal,
        });

        let output_data = match result {
            Ok(todos) => OutputData {
                count: todos.len(),
                items: todos.into_iter().map(TodoView::from).collect(),
            },
            Err(err) => return Output::err(err),
        };

        Output::ok(output_data)
    }
}
