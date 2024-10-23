use crate::http::v1::models::{error::Error, jwt::Claims};
use axum::{body::Body, http, Extension};

use super::auth_model::Me;

pub async fn get_me(Extension(claims): Extension<Claims>) -> Result<http::Response<Body>, Error> {
    let json = serde_json::to_string(&Me {
        username: claims.username,
    })
    .unwrap();
    let response = http::Response::builder()
        .status(200)
        .body(Body::from(json))
        .unwrap();

    Ok(response)
}
