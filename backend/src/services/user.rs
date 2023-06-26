use anyhow::anyhow;
use sqlx::{postgres::PgQueryResult, PgPool};

use crate::api::user::CreateUser;

pub async fn create_user(
    pool: PgPool,
    create_user_info: CreateUser,
) -> Result<PgQueryResult, anyhow::Error> {
    sqlx::query!(
        r#"
INSERT INTO
  users (email, name)
VALUES
  ($1, $2)
    "#,
        create_user_info.email,
        create_user_info.name,
    )
    .execute(&pool)
    .await
    .map_err(|_| anyhow!("Couldn't create user"))
}
