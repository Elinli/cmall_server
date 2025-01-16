use crate::AppState;
use axum::Router;

use super::{setup_role_router, setup_user_router};

pub fn setup_base_router() -> Router<AppState> {
    let user_router = setup_user_router();

    let role_router = setup_role_router();

    let base_router = Router::new()
        .nest("/user", user_router)
        .nest("/role", role_router);

    base_router
}
