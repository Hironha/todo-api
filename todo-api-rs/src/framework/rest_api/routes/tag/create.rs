use std::error::Error;

use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::TagState;
use crate::adapters::controllers::tag::create::CreateTagController;
use crate::adapters::dtos::tag::create::{CreateTagRequest, ParseError};
use crate::adapters::presenters::json::tag::JsonTagPresenter;
use crate::application::dtos::tag::create::CreateTagError;
use crate::application::use_cases::tag::create::CreateTagUseCase;
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
    tracing::info!("Create tag body: {body:?}");

    let input = CreateTagRequest {
        name: body.name,
        description: body.description,
    };

    let presenter = JsonTagPresenter::new();
    let interactor = CreateTagUseCase::new(state.tag_repository);
    let controller = CreateTagController::new(interactor, presenter);
    let output = match controller.run(input).await {
        Ok(output) => output,
        Err(err) => {
            tracing::error!("Create tag error: {err:?}");
            let (status, error) = config_error_response(err);
            return (status, Json(error)).into_response();
        }
    };

    let mut headers = header::HeaderMap::new();
    if let Ok(location) = format!("/tags/{}", output.id).parse::<header::HeaderValue>() {
        headers.insert(header::LOCATION, location);
    }

    (StatusCode::CREATED, headers, Json(output)).into_response()
}

fn config_error_response(error: Box<dyn Error>) -> (StatusCode, ApiError<ValidationError>) {
    if let Some(parse_err) = error.downcast_ref::<ParseError>() {
        let field = match parse_err {
            ParseError::InvalidName(_) => "name",
            ParseError::InvalidDescription(_) => "description",
        };
        let details = ValidationError::new(field, parse_err.to_string());
        let api_error = ApiError::new("ParseError", "Invalid input").with_details(details);
        return (StatusCode::BAD_REQUEST, api_error);
    }

    if let Some(create_err) = error.downcast_ref::<CreateTagError>() {
        return match create_err {
            CreateTagError::DuplicatedName(..) => {
                let api_error = ApiError::new("DuplicatedName", create_err.to_string());
                (StatusCode::CONFLICT, api_error)
            }
            CreateTagError::Internal(..) => {
                let api_error = ApiError::internal();
                (StatusCode::INTERNAL_SERVER_ERROR, api_error)
            }
        };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, ApiError::internal())
}
