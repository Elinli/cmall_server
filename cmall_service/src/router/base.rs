use crate::{signin_handler, signup_handler, AppState};
use axum::{routing::*, Router};

use super::setup_user_router;

pub fn setup_base_router() -> Router<AppState> {
    let user_router = setup_user_router();

    let base_router = Router::new()
        .nest("/user", user_router)
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler));

    base_router
}
