use std::net::TcpListener;

use backend::app::App;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pool = sqlx::PgPool::connect(&dotenv::var("DATABASE_URL")?).await?;
    // TODO: Maybe not use 0.0.0.0 per zero2prod book
    let listener = TcpListener::bind("0.0.0.0:42069").expect("Failed to bind to port");
    Ok(App::new(listener, pool)?.server.await?)
}
