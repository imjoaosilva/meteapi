use crate::http::v1::models::jwt::Claims;
use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Validation};
use crate::KEYS;

pub async fn token_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        let token = extract_token(auth_header).ok_or(StatusCode::UNAUTHORIZED)?;
        
        let claim_data = decode::<Claims>(
            &token,
            &KEYS.decoding,
            &Validation::default(),
        )
        .map_err(|_| {
            println!("Error decoding token");
            StatusCode::UNAUTHORIZED
        })?;

        req.extensions_mut().insert(claim_data.claims);
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    Ok(next.run(req).await)
}

fn extract_token(auth_header: &str) -> Option<&str> {
    let parts: Vec<&str> = auth_header.split_whitespace().collect();
    if parts.len() == 2 && parts[0] == "Bearer" {
        Some(parts[1])
    } else {
        None
    }
}