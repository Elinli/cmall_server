mod app_error;
mod user_error;


pub use app_error::AppError;
pub use user_error::UserError;
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
