use axum::response::{IntoResponse, Response};
use axum::Json;
use base64::DecodeError;
use derive_more::Display;
use hyper::StatusCode;
use image::ImageError;
use serde_json::json;
use sqlx::Error;
use std::fmt::Debug;
use std::num::ParseFloatError;
use std::string::ParseError;

/// Handling all our errors that the backend could run into
#[derive(Display, Debug)]
#[display(fmt = "App Error!")]
pub enum AppError {
    Database(Error),

    /// User errors,
    MissingCredentials,
    UserDoesNotExist,
    UserAlreadyExists,
    InvalidToken,
    InternalServerError,
    InvalidPassword,

    RequestError(reqwest::Error),
    ImageError(ImageError),
    DecodeError(DecodeError),

    ParseError(ParseFloatError),

    Any(anyhow::Error),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Any(err) => {
                let message = format!("Internal server error! {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::Database(err) => {
                let message = format!("Database error! {}", err);
                (StatusCode::SERVICE_UNAVAILABLE, message)
            }
            AppError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                "Missing or incorrect credentials.".to_string(),
            ),
            AppError::UserAlreadyExists => {
                (StatusCode::UNAUTHORIZED, "User already exists.".to_string())
            }
            AppError::UserDoesNotExist => (
                StatusCode::UNAUTHORIZED,
                "User does not exists.".to_string(),
            ),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token.".to_string()),
            AppError::InternalServerError => (
                StatusCode::UNAUTHORIZED,
                "Something very very bad happened....".to_string(),
            ),
            AppError::InvalidPassword => (
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            ),
            AppError::RequestError(err) => {
                let message = format!("Error making request: {}", err);
                (StatusCode::BAD_REQUEST, message)
            }
            AppError::ImageError(err) => {
                let message = format!("Image error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::DecodeError(err) => {
                let message = format!("Decoding error: {}", err);
                (StatusCode::SERVICE_UNAVAILABLE, message)
            }
            AppError::ParseError(err) => {
                let message = format!("Parsing Distance error: {}", err);
                (StatusCode::SERVICE_UNAVAILABLE, message)

            }
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}

/// Implementing from trait for AppError from sqlxError
impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Database(value)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::RequestError(value)
    }
}

impl From<ImageError> for AppError {
    fn from(value: ImageError) -> Self {
        AppError::ImageError(value)
    }
}

impl From<DecodeError> for AppError {
    fn from(value: DecodeError) -> Self {
        AppError::DecodeError(value)
    }
}

impl From<ParseFloatError> for AppError {
    fn from(value: ParseFloatError) -> Self {
        AppError::ParseError(value)
    }
}
