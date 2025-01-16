use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use cmall_core::User;
use serde::{Deserialize, Serialize};

use crate::{
    error::{ErrorOutput, AppError},
    AppState, CreateUser, LoginUser,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthOutput {
    token: String,
    user: User,
}

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.create_user(&input).await?;
    let token = state.secret_key.sign(user.clone())?;
    let body = Json(AuthOutput { token, user });
    Ok((StatusCode::CREATED, body))
}

pub async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.verify_user(&input).await?;

    match user {
        Some(user) => {
            let token = state.secret_key.sign(user.clone())?;
            Ok((StatusCode::OK, Json(AuthOutput { token, user })).into_response())
        }
        None => {
            let body = Json(ErrorOutput::new("Invalid Credentials"));
            Ok((StatusCode::FORBIDDEN, body).into_response())
        }
    }
}
