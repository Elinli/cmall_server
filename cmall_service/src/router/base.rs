use crate::AppState;
use axum::Router;

use super::setup_user_router;

pub fn setup_base_router() -> Router<AppState> {
    let user_router = setup_user_router();

    let base_router = Router::new().nest("/user", user_router);

    base_router
}
