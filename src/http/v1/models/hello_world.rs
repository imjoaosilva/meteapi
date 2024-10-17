use serde::Serialize;

#[derive(Serialize)]
pub enum HelloWorld {
    Message(String),
}
