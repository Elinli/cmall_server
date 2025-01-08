use crate::{
    // create_user_handler,
    // delete_user_handler,
    create_user_handler,
    delete_user_handler,
    export_users_handler,
    get_user_handler,
    list_search_user_handler,
    update_user_handler,
    AppState, // list_user_handler,
              // update_user_handler,
};
use axum::{routing::*, Router};

pub fn setup_user_router() -> Router<AppState> {
    let user_router = Router::new()
        .route(
            "/:id",
            get(get_user_handler)
                .delete(delete_user_handler)
                .post(update_user_handler),
        )
        .route("/export", post(export_users_handler))
        .route("/", get(list_search_user_handler).post(create_user_handler));

    user_router
}
