use crate::services::auth::compute_hash;
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
  users (email, firstname, lastname, password)
VALUES
  ($1, $2, $3, $4)
    "#,
        create_user_info.email,
        create_user_info.firstname,
        create_user_info.lastname,
        compute_hash(&create_user_info.password)
    )
    .execute(&pool)
    .await
    .map_err(|_| anyhow!("Couldn't create user"))
}
