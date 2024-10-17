use axum::Json;
use crate::http::v1::models::{error::Error, hello_world::HelloWorld};

pub async fn hello_world() -> Result<Json<HelloWorld>, Error> {
    Ok(Json(HelloWorld::Message("Hello, World!".to_string())))
}