mod create;

use axum::extract::FromRef;
use axum::routing::post;
use axum::Router;
use sqlx::{Pool, Postgres};

use create::create_tag;

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
        .with_state(state)
}
