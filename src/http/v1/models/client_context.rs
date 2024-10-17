use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct ClientContext {
    pub db: Pool<MySql>
}