use std::error::Error;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TodoState;
use crate::adapters::controllers::todo::list::ListController;
use crate::adapters::dtos::todo::list::{ListRequest, ParseError};
use crate::application::dtos::todo::list::ListTodoError;
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
    let req = ListRequest {
        page: query.page,
        per_page: query.per_page,
        title: query.title,
    };

    tracing::info!("List todos request: {req:?}");

    let controller = ListController::new(state.todo_repository);
    let output = match controller.run(req).await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("List todos error: {err:?}");
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::OK, Json(output)).into_response()
}

fn config_error_response(error: Box<dyn Error>) -> (StatusCode, ApiError<ValidationError>) {
    if let Some(parse_err) = error.downcast_ref::<ParseError>() {
        let field = match parse_err {
            ParseError::InvalidPage => "page",
            ParseError::InvalidPerPage => "perPage",
            ParseError::Title(_) => "title",
        };
        let details = ValidationError::new(field, parse_err.to_string());
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(list_err) = error.downcast_ref::<ListTodoError>() {
        return match list_err {
            ListTodoError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
