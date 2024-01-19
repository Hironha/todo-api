use std::error::Error;

use crate::adapters::dtos::todo::bind_tags::BindTagsRequest;
use crate::adapters::dtos::todo::create::CreateRequest;
use crate::adapters::dtos::todo::delete::DeleteRequest;
use crate::adapters::dtos::todo::find::FindRequest;
use crate::adapters::dtos::todo::list::{ListRequest, ListResponse};
use crate::adapters::dtos::todo::update::UpdateRequest;
use crate::adapters::presenters::todo::TodoPresenter;
use crate::application::repositories::tag::TagRepository;
use crate::application::repositories::todo::TodoRepository;
use crate::application::use_cases::todo::bind_tags::BindTodoTagsUseCase;
use crate::application::use_cases::todo::create::CreateTodoUseCase;
use crate::application::use_cases::todo::delete::DeleteTodoUseCase;
use crate::application::use_cases::todo::find::FindTodoUseCase;
use crate::application::use_cases::todo::list::ListTodosUseCase;
use crate::application::use_cases::todo::update::UpdateTodoUseCase;

#[derive(Clone, Debug)]
pub struct TodoController<T, S>
where
    T: TodoRepository,
    S: TagRepository,
{
    todo_repository: T,
    tag_repository: S,
}

impl<T, S> TodoController<T, S>
where
    T: TodoRepository,
    S: TagRepository,
{
    pub fn new(todo_repository: T, tag_repository: S) -> Self {
        Self {
            todo_repository,
            tag_repository,
        }
    }

    pub async fn bind_tags(self, req: BindTagsRequest) -> Result<(), Box<dyn Error>> {
        let input = req.parse()?;

        BindTodoTagsUseCase::new(self.todo_repository, self.tag_repository)
            .exec(input)
            .await
            .map_err(Box::from)
    }

    pub async fn create(self, req: CreateRequest) -> Result<TodoPresenter, Box<dyn Error>> {
        let input = req.parse()?;

        CreateTodoUseCase::new(self.todo_repository)
            .exec(input)
            .await
            .map(TodoPresenter::from_entity)
            .map_err(Box::from)
    }

    pub async fn delete(self, req: DeleteRequest) -> Result<(), Box<dyn Error>> {
        let todo_id = req.parse()?;

        DeleteTodoUseCase::new(self.todo_repository)
            .exec(todo_id)
            .await
            .map_err(Box::from)
    }

    pub async fn find(self, req: FindRequest) -> Result<TodoPresenter, Box<dyn Error>> {
        let todo_id = req.parse()?;

        FindTodoUseCase::new(self.todo_repository)
            .exec(todo_id)
            .await
            .map(TodoPresenter::from_entity)
            .map_err(Box::from)
    }

    pub async fn list(self, req: ListRequest) -> Result<ListResponse, Box<dyn Error>> {
        let input = req.parse()?;

        ListTodosUseCase::new(self.todo_repository)
            .exec(input)
            .await
            .map(ListResponse::from_list)
            .map_err(Box::from)
    }

    pub async fn update(self, req: UpdateRequest) -> Result<(), Box<dyn Error>> {
        let input = req.parse()?;

        UpdateTodoUseCase::new(self.todo_repository)
            .exec(input)
            .await
            .map_err(Box::from)
    }
}
