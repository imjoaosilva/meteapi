use axum::{body::Body, http, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing credentials")]
    MissingCredentials(),

    #[error("Wrong credentials")]
    WrongCredentials(),
}

impl IntoResponse for Error {
    fn into_response(self) -> http::Response<Body> {
        match self {
            Error::MissingCredentials() => {
                let error = r#"{ "status": 400, "message": "Missing Credentials" }"#;

                http::Response::builder()
                    .status(400)
                    .body(Body::from(error))
                    .unwrap()
            }
            Error::WrongCredentials() => {
                let error = r#"{ "status": 401, "message": "Wrong Credentials" }"#;

                http::Response::builder()
                    .status(401)
                    .body(Body::from(error))
                    .unwrap()
            }

        }
    }
}