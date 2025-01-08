use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use thiserror::Error;
use super::ErrorOutput;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),


}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HttpHeaderError(_) => StatusCode::BAD_REQUEST,
            
        };
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
