use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use rust_xlsxwriter::XlsxError;
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub message: String,
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            message: error.into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    // app error
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),

    // user error
    #[error("user alredy existed: {0}")]
    UserAlreadyExisted(String),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("export user error: {0}")]
    ExportUserError(#[from] XlsxError),

    // role error
    #[error("role already existed: {0}")]
    RoleAlreadyExisted(String),

    // common error
    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HttpHeaderError(_) => StatusCode::BAD_REQUEST,
            // user error
            Self::UserAlreadyExisted(_) => StatusCode::CONFLICT,
            Self::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ExportUserError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // common error
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
