mod user;
use serde::{Deserialize, Serialize};
pub use user::*;

mod auth;
pub use auth::*;

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
