use serde::{Deserialize, Serialize};

mod user;
pub use user::*;

mod auth;
pub use auth::*;

mod role;
pub use role::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordOutput<T> {
    body: Vec<T>,
    total: i64,
}
impl<T> RecordOutput<T> {
    pub fn new(body: Vec<T>, total: i64) -> Self {
        Self { body, total }
    }
}
