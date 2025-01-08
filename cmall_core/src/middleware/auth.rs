
use axum::{
    extract::{FromRequestParts, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use tracing::warn;

use super::TokenVerify;

#[derive(Debug, Deserialize)]
struct Params {
    token: String,
}

pub async fn verify_token<T>(State(state): State<T>, req: Request, next: Next) -> Response
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    let (mut parts, body) = req.into_parts();
    let token =
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
            Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
            Err(e) => {
                if e.is_missing() {
                    match Query::<Params>::from_request_parts(&mut parts, &state).await {
                        Ok(query) => query.token.clone(),
                        Err(e) => {
                            let msg = format!("parse query params failed: {}", e);
                            warn!("{}", msg);
                            return (StatusCode::UNAUTHORIZED, msg).into_response();
                        }
                    }
                } else {
                    let msg = format!("parse bearer token failed: {}", e);
                    warn!(msg);
                    return (StatusCode::UNAUTHORIZED, msg).into_response();
                }
            }
        };
    let req = match state.verify(&token) {
        Ok(user) => {
            let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(user);
            req
        }
        Err(e) => {
            let msg = format!("verify token error: {:?}", e);
            warn!(msg);
            return (StatusCode::FORBIDDEN, msg).into_response();
        }
    };
    next.run(req).await
}
