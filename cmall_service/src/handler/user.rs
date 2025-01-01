use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use crate::error::UserError;
use crate::AppState;

// pub fn list_user_handler(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
// ) -> Result<impl IntoResponse, UserError> {
//     // let user = state.
// }

pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, UserError> {
    let user = state.find_user_by_id(id).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(UserError::NotFound(id.to_string())),
    }
}

// pub fn update_user_handler() -> impl IntoResponse {
//     "Hello, World! update_user_handler"
// }
// pub fn delete_user_handler() -> impl IntoResponse {
//     "Hello, World! delete_user_handler"
// }

// // create_user_handler
// pub fn create_user_handler() -> impl IntoResponse {
//     "Hello, World! create_user_handler"
// }
