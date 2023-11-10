use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use openai_api_rs::v1::error::APIError;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Date Error: {0}")]
    DateError(String),

    #[error("Backend error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Invalid Input")]
    JsonRejection(#[from] JsonRejection),

    #[error("Value parsing error")]
    ValueError,

    #[error("Open AI Error {0}")]
    OpenAiError(#[from] APIError),

    #[error("Io Error")]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use Error::*;
        let status = match self {
            DateError(_) => StatusCode::BAD_REQUEST,
            ReqwestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            JsonRejection(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ValueError => StatusCode::INTERNAL_SERVER_ERROR,
            OpenAiError(_) => StatusCode::BAD_REQUEST,
            IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({"error": self.to_string()}))).into_response()
    }
}
