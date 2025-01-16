use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "effect_status", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum EffectStatus {
    Enable,
    Disable,
}


impl fmt::Display for EffectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EffectStatus::Enable => write!(f, "enable"),
            EffectStatus::Disable => write!(f, "disable"),
        }
    }
}


#[derive(Debug, Clone, FromRow, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub status: EffectStatus,
    pub description: String,
    pub create_time: DateTime<Utc>,
    pub create_by: String,
    pub update_time: DateTime<Utc>,
    pub update_by: String,
}