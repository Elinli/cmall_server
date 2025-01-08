use std::fs;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use cmall_core::{User, UserStatus};
use rust_xlsxwriter::{Format, Workbook};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{error::UserError, CreateUser, UpdateUser};
use crate::{AppState, RecordOutput};

// #[serde(deny_unknown_fields)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[serde(expecting = "params is error")]
#[serde(rename_all = "camelCase")]
pub struct SearchUser {
    #[serde(default)]
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<UserStatus>,
    pub page_num: i64,
    pub page_size: i64,
}

pub async fn list_search_user_handler(
    State(state): State<AppState>,
    Query(input): Query<SearchUser>,
) -> Result<impl IntoResponse, UserError> {
    info!("search user: {:?}", input);
    // let user = state.
    let (users, total_count) = state
        .find_user_by_conditions(
            input.username.as_deref(),
            input.email.as_deref(),
            input.phone.as_deref(),
            input.status,
            input.page_num,
            input.page_size,
        )
        .await?;
    Ok(Json(RecordOutput::new(users, total_count)))
}

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

pub async fn update_user_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateUser>,
) -> Result<impl IntoResponse, UserError> {
    let user = state.update_user(id, &input).await?;
    Ok((StatusCode::OK, Json(user)))
}

// create_user_handler
pub async fn create_user_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, UserError> {
    info!("create_user_handler {:?}", input);
    let user = state.create_user(&input).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, UserError> {
    info!("delete_user_handler {:?}", id);
    let result = state.delete_user(id).await?;
    let success = Json(result);

    Ok((StatusCode::OK, success))
}

pub async fn export_users_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, UserError> {
    info!("export_file_handler");
    let users = state.find_users().await?;

    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();
    let date_format = Format::new().set_num_format("d mmm yyyy");

    // Iterate over the data and write it out row by row.
    let mut row = 1;
    for user in &users {
        worksheet.write(row, 0, user.id)?;
        worksheet.write(row, 1, user.username.clone())?;
        worksheet.write(row, 2, user.phone.clone())?;
        worksheet.write(row, 3, user.email.clone())?;
        worksheet.write(row, 4, user.status.to_string())?;
        worksheet.write_with_format(
            row,
            5,
            user.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            &date_format,
        )?;
        row += 1;
    }
    let bold = Format::new().set_bold();
    // Write some column headers.
    worksheet.write_with_format(0, 0, "编号", &bold)?;
    worksheet.write_with_format(0, 1, "名称", &bold)?;
    worksheet.write_with_format(0, 2, "电话", &bold)?;
    worksheet.write_with_format(0, 3, "邮箱", &bold)?;
    worksheet.write_with_format(0, 4, "状态", &bold)?;
    worksheet.write_with_format(0, 5, "创建时间", &bold)?;

    worksheet.set_column_width(1, 15)?;
    worksheet.set_column_width(2, 15)?;
    worksheet.set_column_width(3, 15)?;
    worksheet.set_column_width(5, 25)?;

    // Save the file to disk.
    let path = "user.xlsx";
    workbook.save(path)?;

    info!("Path: {}", path);
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    // TODO: streaming
    let body = fs::read(path)?;
    info!("Body: {:?}", body);
    let mut headers = HeaderMap::new();
    headers.insert("content-type", mime.to_string().parse().unwrap());
    Ok((headers, body))
}
