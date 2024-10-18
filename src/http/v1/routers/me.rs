use axum::{middleware, routing::get, Router};

use crate::http::v1::middlewares::token::token_middleware;

use super::get_me;

pub fn router() -> Router {
    Router::new().route("/@me", get(get_me).route_layer(middleware::from_fn(token_middleware)))
}
