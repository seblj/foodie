use std::env;

use sqlx::{Pool, Postgres};

pub async fn connect() -> Result<Pool<Postgres>, sqlx::Error> {
    let host = env::var(format!("POSTGRES_HOST")).unwrap();
    let username = env::var(format!("POSTGRES_USER")).unwrap();
    let password = env::var(format!("POSTGRES_PASSWORD")).unwrap();
    let url = format!("postgres://{}:{}@{}", username, password, host);

    Ok(sqlx::PgPool::connect(&url).await?)
}
