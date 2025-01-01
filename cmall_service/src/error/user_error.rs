use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use thiserror::Error;

use super::ErrorOutput;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user alredy existed: {0}")]
    UserAlreadyExisted(String),

    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("not found: {0}")]
    NotFound(String),
    // #[error("unauthorized")]
    // Unauthorized,

    // #[error("forbidden")]
    // Forbidden,

    // #[error("bad request")]
    // BadRequest,

    // #[error("internal server error")]
    // InternalServerError,

    // #[error("not implemented")]
    // NotImplemented,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            Self::UserAlreadyExisted(_) => StatusCode::INTERNAL_SERVER_ERROR,

            Self::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,

            Self::NotFound(_) => StatusCode::NOT_FOUND,

            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
