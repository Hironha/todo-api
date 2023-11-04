use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::list::ListController;
use crate::adapters::dtos::todo::list::{ListRequest, ParseError, RunError};
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct QueryParams {
    page: Option<u32>,
    #[serde(rename(deserialize = "perPage"))]
    per_page: Option<u32>,
    title: Option<String>,
}

pub(super) async fn list_todo(
    State(state): State<TodoState>,
    Query(query): Query<QueryParams>,
) -> impl IntoResponse {
    let input = ListRequest {
        page: query.page,
        per_page: query.per_page,
        title: query.title,
    };
    let controller = ListController::new(state.todo_repository);

    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: RunError) -> (StatusCode, ApiError<ValidationError>) {
    match error {
        RunError::Parsing(err) => {
            let field = match err {
                ParseError::InvalidPage => "page",
                ParseError::InvalidPerPage => "perPage",
                ParseError::Title(_) => "title",
            };
            let details = ValidationError::new(field, err.description());
            let error = ApiError::new("LTD-001", "Invalid input").with_details(details);
            (StatusCode::BAD_REQUEST, error)
        }
        RunError::Internal => {
            let error = ApiError::new("LTD-001", "Internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        }
    }
}
