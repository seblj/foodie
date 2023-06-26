use anyhow::anyhow;
use sqlx::{postgres::PgQueryResult, PgPool};

use crate::api::auth::{User, UserInfo};

pub async fn create_user(
    pool: &PgPool,
    create_user_info: &UserInfo,
) -> Result<PgQueryResult, anyhow::Error> {
    sqlx::query!(
        r#"
INSERT INTO
  users (email, name)
VALUES
  ($1, $2)
on conflict (email) do nothing;
    "#,
        create_user_info.email,
        create_user_info.name,
    )
    .execute(pool)
    .await
    .map_err(|_| anyhow!("Couldn't create user"))
}

pub async fn get_user(pool: &PgPool, email: String) -> Result<User, anyhow::Error> {
    let user = sqlx::query!(
        r#"
SELECT
  *
FROM
  users
WHERE
  email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        id: user.id,
        name: user.name,
        email: user.email,
    })
}
