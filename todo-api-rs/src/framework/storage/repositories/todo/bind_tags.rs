use sqlx::types::time::OffsetDateTime;
use sqlx::PgConnection;

use crate::application::repositories::todo::bind_tags::{BindTagsError, BindTagsPayload};

async fn bind_tags(conn: &mut PgConnection, payload: BindTagsPayload) -> Result<(), BindTagsError> {
    todo!();
}
