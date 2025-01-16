use crate::{create_role_handler, delete_role_handler, list_role_handler, update_role_handler, AppState};
use axum::{routing::*, Router};

pub fn setup_role_router() -> Router<AppState> {
    let role_router = Router::new()
        .route(
            "/:id",
            // get(get_role_handler)
            delete(delete_role_handler).post(update_role_handler),
        )
        // .route("/export", post(export_roles_handler))
        .route("/", get(list_role_handler).post(create_role_handler));

    role_router
}
