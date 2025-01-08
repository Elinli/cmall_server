mod config;
mod error;
mod handler;
mod models;
mod router;
mod serde_error;

use anyhow::Context;
use axum::{
    http::Method,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use core::fmt;
use error::AppError;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
// use sqlx_db_tester::TestPg;
use std::{ops::Deref, sync::Arc};
use tokio::fs;

use cmall_core::{verify_token, DecodingKeyPair, EncodingKeyPair, TokenVerify, User};
pub use config::*;
pub use handler::*;
pub use models::*;
pub use router::*;

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) secret_key: EncodingKeyPair,
    pub(crate) public_key: DecodingKeyPair,
    pub(crate) pool: PgPool,
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir)
            .await
            .context("Create base_dir failed")?;

        let secret_key = EncodingKeyPair::load_secret_key(&config.auth.secret_key)
            .context("Load secret_key failed")?;

        let public_key = DecodingKeyPair::load_public_key(&config.auth.public_key)
            .context("Load public_key failed")?;

        let pool = PgPool::connect(&config.server.db_url)
            .await
            .context("Connect to database failed")?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                secret_key,
                public_key,
                pool,
            }),
        })
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

pub fn setup_router(state: AppState) -> Result<Router, AppError> {
    let origins = ["http://localhost:5173".parse().unwrap()];

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_origin(origins)
        .allow_headers(Any);
    let base_router = setup_base_router()
        .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .layer(cors);
    let cmall_router = Router::new()
        .route("/", get(index_handler))
        .nest("/api/v1", base_router)
        .with_state(state);
    Ok(cmall_router)
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.public_key.verify(token)?)
    }
}

async fn index_handler() -> impl IntoResponse {
    "Weclome To Reny Cmall!"
}

#[cfg(feature = "test-util")]
mod test_util {
    use super::*;

    use sqlx::{Executor, PgPool};
    use sqlx_db_tester::TestPg;

    impl AppState {
        pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
            let config = AppConfig::load_config()?;

            let secret_key = EncodingKeyPair::load_secret_key(&config.auth.secret_key)
                .context("Load secret_key failed")?;

            let public_key = DecodingKeyPair::load_public_key(&config.auth.public_key)
                .context("Load public_key failed")?;

            let post = config
                .server
                .db_url
                .rfind('/')
                .expect("Invalid database url");

            let server_url = &config.server.db_url[..post];

            let (tdb, pool) = get_test_pool(Some(server_url)).await;

            let state = Self {
                inner: Arc::new(AppStateInner {
                    config,
                    secret_key,
                    public_key,
                    pool,
                }),
            };
            Ok((tdb, state))
        }
    }

    pub async fn get_test_pool(server_url: Option<&str>) -> (TestPg, PgPool) {
        let url = match server_url {
            Some(url) => url.to_string(),
            None => "postgres://postgres:admin@localhost:5432".to_string(),
        };
        let tdb = TestPg::new(url, std::path::Path::new("../migrations"));
        let pool = tdb.get_pool().await;

        let sql = include_str!("../fixtures/test.sql").split(";");
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");
        (tdb, pool)
    }
}
