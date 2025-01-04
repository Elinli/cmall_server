use crate::{
    // create_user_handler,
    // delete_user_handler,
    get_user_handler,
    AppState,
    // list_user_handler,
    // update_user_handler,
};
use axum::{routing::*, Router};

pub fn setup_user_router() -> Router<AppState> {
    let user_router = Router::new().route("/:id", get(get_user_handler));

    user_router
}
