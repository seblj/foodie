use actix_web::web;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{Pool, Postgres};

pub fn compute_hash(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&format!("qGmLwByUv_{}", input).as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

pub struct User {
    pub id: i64,
}

pub async fn authenticate(
    pool: web::Data<Pool<Postgres>>,
    login_info: LoginInfo,
) -> Result<User, anyhow::Error> {
    let pool = pool.get_ref();
    let user = sqlx::query!(
        r#"
SELECT
  *
FROM
  users
WHERE
  email = $1
        "#,
        login_info.email
    )
    .fetch_one(pool)
    .await?;

    if user.password == compute_hash(login_info.password) {
        return Ok(User { id: user.id });
    }

    Err(anyhow::anyhow!("Not authenticated"))
}
