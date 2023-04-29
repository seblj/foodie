use argon2::Config;
use axum_login::{secrecy::SecretVec, AuthUser};
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

impl AuthUser<i64> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        // I don't understand why this trait MUST have a `get_password_hash` function.
        SecretVec::new("".into())
    }
}

pub fn compute_hash(input: &str) -> String {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(input.as_bytes(), &salt, &config)
        .expect("Need to be able to hash password")
}

pub fn verify_password(encoded: &str, pass: &str) -> Result<bool, anyhow::Error> {
    argon2::verify_encoded(encoded, pass.as_bytes()).map_err(|e| e.into())
}

pub async fn authenticate(
    pool: Pool<Postgres>,
    login_info: LoginInfo,
) -> Result<User, anyhow::Error> {
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
    .fetch_one(&pool)
    .await?;

    if verify_password(&user.password, &login_info.password)? {
        return Ok(User {
            id: user.id,
            email: user.email,
            firstname: user.firstname,
            lastname: user.lastname,
        });
    }

    Err(anyhow::anyhow!("Not authenticated"))
}
