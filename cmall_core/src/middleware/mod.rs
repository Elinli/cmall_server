mod auth;
use std::fmt;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

pub use auth::verify_token;

use crate::User;

pub trait TokenVerify {
    type Error: fmt::Debug;

    fn verify(&self, token: &str) -> Result<User, Self::Error>;
}

pub fn setup_layer(app: Router) -> Router {
    app.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    )
                    .on_failure(
                        DefaultOnFailure::new()
                            .level(Level::ERROR)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            )
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true)),
    )
}
