use super::User;
use sqlx::{self, MySql, Pool};

pub async fn get_user_by_username(
    pool: Pool<MySql>,
    username: &String,
) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT *
        FROM users
        WHERE username = ?
        "#,
        username
    )
    .fetch_optional(&pool)
    .await?;

    Ok(user)
}
