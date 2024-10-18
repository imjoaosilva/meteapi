use anyhow::Ok;
use axum::{Extension, Router};
use models::client_context::ClientContext;
use routers::{auth, helloworld, me};
use tower_http::cors::CorsLayer;

pub mod controllers;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod routers;

pub async fn serve(context: ClientContext, port: String) -> anyhow::Result<()> {
    let app = api_router()
        .layer(Extension(context))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("🚀 Server running on port {}", port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn api_router() -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .nest("/", auth::router())
            .nest("/", me::router())
            .nest("/", helloworld::router()),
    )
}
