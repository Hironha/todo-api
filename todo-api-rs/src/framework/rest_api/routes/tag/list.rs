use std::error::Error;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use super::TagState;
use crate::adapters::controllers::tag::list::ListController;
use crate::application::dtos::tag::list::ListTagError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

pub(super) async fn list_tags(State(state): State<TagState>) -> impl IntoResponse {
    let controller = ListController::new(state.tag_repository);
    let output = match controller.run().await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("List tags error: {err:?}");
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: Box<dyn Error>) -> (StatusCode, ApiError<ValidationError>) {
    if let Some(list_err) = error.downcast_ref::<ListTagError>() {
        return match list_err {
            ListTagError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
