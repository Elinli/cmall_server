use cmall_core::User;

use anyhow::Result;
use cmall_service::{setup_router, AppConfig, AppState, CreateUser};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let config = AppConfig::load_config()?;

    let addr = format!("{}:{}", config.server.host, config.server.port);

    let state = AppState::try_new(config).await.unwrap();

    let app = setup_router(state)?;
  
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    
    Ok(())
}
