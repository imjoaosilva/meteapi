use axum::{routing::post, Router};

use super::login;

pub fn router() -> Router {
    Router::new().route("/login", post(login))
}
