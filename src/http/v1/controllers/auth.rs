use super::{auth_model::{self, LoginResponse}, client_context::ClientContext, error::Error, user};
use axum::{body::Body, extract::rejection::JsonRejection, http, Extension, Json};

pub async fn login(
    client: Extension<ClientContext>,
    payload: Result<Json<auth_model::Login>, JsonRejection>,
) -> Result<http::Response<Body>, Error> {
    let payload = match payload {
        Ok(payload) => payload,
        Err(_) => return Err(Error::MissingCredentials()),
    };

    match user::get_user_by_username(client.db.clone(), &payload.username).await {
        Ok(Some(user)) => {
            if user.password == payload.password {
                let json = serde_json::to_string(&LoginResponse {
                    status: 200,
                    message: "Login successful".to_string(),
                    token: "tester".to_string(),
                })
                .unwrap();
                let response = http::Response::builder()
                    .status(200)
                    .body(Body::from(json))
                    .unwrap();

                Ok(response)
            } else {
                Err(Error::WrongCredentials())
            }
        }
        Ok(None) => {
            Err(Error::WrongCredentials())
        }
        Err(_) => {
            Err(Error::MissingCredentials())
        }
    }
}