use once_cell::sync::Lazy;
use http::v1::models::jwt::Keys;
pub mod http;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});