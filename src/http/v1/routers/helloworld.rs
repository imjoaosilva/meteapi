use axum::{routing::get, Router};

use super::hello_world;

pub fn router() -> Router {
    Router::new().route("/helloworld", get(hello_world))
}
