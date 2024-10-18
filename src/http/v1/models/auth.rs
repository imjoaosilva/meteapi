use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: i32,
    pub message: String,
    pub token: String,
    pub username: String
}

#[derive(Serialize, Deserialize)]
pub struct Me {
    pub username: String
}