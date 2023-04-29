use std::env;

use sqlx::{Pool, Postgres};

pub async fn connect() -> Result<Pool<Postgres>, sqlx::Error> {
    let host = env::var("POSTGRES_HOST").unwrap();
    let username = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();
    let url = format!("postgres://{}:{}@{}", username, password, host);

    sqlx::PgPool::connect(&url).await
}
