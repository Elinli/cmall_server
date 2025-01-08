use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum UserStatus {
    Active,
    Off,
    Offline,
    Online,
}
impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserStatus::Active => write!(f, "Active"),
            UserStatus::Off => write!(f, "off"),
            UserStatus::Offline => write!(f, "offline"),
            UserStatus::Online => write!(f, "online"),
            // 添加其他状态的匹配
        }
    }
}

#[derive(Debug, Clone, FromRow, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub dept_id: i64,
    pub username: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub email: String,
    pub phone: String,
    pub avatar: String,
    pub status: UserStatus,
    pub roles: Vec<i64>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}


#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq)]
pub struct SignUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub avatar: String,
    pub status: UserStatus,
    pub dept_id: i64,
    pub roles: Vec<i64>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

impl User {
    pub fn new(id: i64, username: &str, email: &str, phone: &str) -> Self {
        Self {
            id,
            dept_id: 1,
            username: username.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
            password_hash: None,
            avatar: "default".to_string(),
            status: UserStatus::Active,
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            roles: [1, 2].to_vec(),
        }
    }
}
