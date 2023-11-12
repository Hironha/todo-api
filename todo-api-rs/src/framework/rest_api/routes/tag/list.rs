use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use super::TagState;
use crate::adapters::controllers::tag::list::ListController;
use crate::adapters::dtos::tag::list::RunError;
use crate::application::dtos::tag::list::ListTagError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

pub(super) async fn list_tags(State(state): State<TagState>) -> impl IntoResponse {
    let controller = ListController::new(state.tag_repository);
    let output = match controller.run().await {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(&err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: &RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Listing(err) => match err {
            ListTagError::Repository(err) => {
                tracing::error!("list tags repository error: {err}");
                let error = ApiError::new("LTG-001", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
        },
    }
}
