mod create;
mod delete;
mod find;

use axum::extract::FromRef;
use axum::routing::{get, post};
use axum::Router;
use sqlx::{Pool, Postgres};

use create::create_tag;
use delete::delete_tag;
use find::find_tag;

use crate::framework::storage::TagStore;

#[derive(Clone, FromRef)]
pub struct TagState {
    tag_store: TagStore,
}

pub fn create_tag_router(pool: Pool<Postgres>) -> Router {
    let state = TagState {
        tag_store: TagStore::new(pool),
    };

    Router::new()
        .route("/tags", post(create_tag))
        .route("/tags/:id", get(find_tag).delete(delete_tag))
        .with_state(state)
}
