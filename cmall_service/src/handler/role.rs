// generate handlers from role model

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use cmall_core::{EffectStatus, User};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{error::AppError, AppState, OperateRole, RecordOutput};

// #[serde(deny_unknown_fields)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[serde(expecting = "params is error")]
#[serde(rename_all = "camelCase")]
pub struct SearchRole {
    pub code: Option<String>,
    pub status: Option<EffectStatus>,
    pub page_num: i64,
    pub page_size: i64,
}
pub async fn create_role_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<OperateRole>,
) -> Result<impl IntoResponse, AppError> {
    let role = state.create_role(&input, user.username).await?;
    Ok((StatusCode::CREATED, Json(role)))
}

pub async fn list_role_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Query(input): Query<SearchRole>,
) -> Result<impl IntoResponse, AppError> {
    info!("list_role_handler {:?}", input);
    let (roles, total_count) = state
        .find_role_by_condition(
            input.code.as_deref(),
            input.status,
            input.page_num,
            input.page_size,
        )
        .await?;

    Ok(Json(RecordOutput::new(roles, total_count)))
}

pub async fn update_role_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<OperateRole>,
) -> Result<impl IntoResponse, AppError> {
    let role = state.update_role(id, &input).await?;
    Ok((StatusCode::OK, Json(role)))
}

pub async fn delete_role_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    info!("delete_role_handler {:?}", id);
    let result = state.delete_role(id).await?;
    let success = Json(result);

    Ok((StatusCode::OK, success))
}
