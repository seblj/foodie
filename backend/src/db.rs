use sqlx::{Pool, Postgres};

pub async fn connect() -> Result<Pool<Postgres>, sqlx::Error> {
    let url = dotenv::var("DATABASE_URL").unwrap();
    sqlx::PgPool::connect(&url).await
}
