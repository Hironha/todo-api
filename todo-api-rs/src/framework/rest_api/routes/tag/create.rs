use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::create::CreateController;
use crate::adapters::dtos::tag::create::{CreateRequest, ParseError, RunError};
use crate::application::dtos::tag::create::CreateTagError;
use crate::framework::rest_api::error::{ApiError, ValidationError};

#[derive(Clone, Debug, Deserialize)]
pub(super) struct CreateBody {
    name: Option<String>,
    description: Option<String>,
}

pub(super) async fn create_tag(
    State(state): State<TagState>,
    Json(body): Json<CreateBody>,
) -> impl IntoResponse {
    tracing::info!("create tag body: {body:?}");

    let input = CreateRequest {
        name: body.name,
        description: body.description,
    };

    let controller = CreateController::new(state.tag_repository);
    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            let (status, error) = config_error_response(&err);
            return (status, Json(error)).into_response();
        }
    };

    (StatusCode::CREATED, Json(output)).into_response()
}

fn config_error_response(error: &RunError) -> (StatusCode, ApiError<ValidationError>) {    
    match error {
        RunError::Parsing(parse_err) => {
            let field = match parse_err {
                ParseError::EmptyName | ParseError::InvalidName(_) => "name",
                ParseError::InvalidDescription(_) => "description",
            };
            let details = ValidationError::new(field, parse_err.to_string());
            let api_error = ApiError::new("CTG-001", error.to_string()).with_details(details);
            (StatusCode::BAD_REQUEST, api_error)
        }
        RunError::Creating(create_err) => match create_err {
            CreateTagError::Repository(repository_err) => {
                tracing::error!("create tag repository error: {repository_err}");
                let api_error = ApiError::new("CTG-002", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        },
    }
}
