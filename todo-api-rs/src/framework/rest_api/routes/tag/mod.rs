mod create;
mod delete;
mod find;
mod list;
mod update;

use axum::extract::FromRef;
use axum::routing::get;
use axum::Router;
use sqlx::{Pool, Postgres};

use create::create_tag;
use delete::delete_tag;
use find::find_tag;
use list::list_tags;
use update::update_tag;

use crate::framework::storage::repositories::tag::TagRepository;

#[derive(Clone, FromRef)]
pub struct TagState {
    tag_repository: TagRepository,
}

pub fn create_tag_router(pool: Pool<Postgres>) -> Router {
    let state = TagState {
        tag_repository: TagRepository::new(pool),
    };

    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:id", get(find_tag).delete(delete_tag).put(update_tag))
        .with_state(state)
}
