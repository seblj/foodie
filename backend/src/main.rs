use backend::app::App;
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = ConnectOptions::new("postgres://postgres:postgres@localhost:5432/postgres");
    let db = Database::connect(opt).await?;
    // TODO: Maybe not use 0.0.0.0 per zero2prod book
    let app = App::new(db).await?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .expect("Failed to bind to port");
    println!("Server running on {}", listener.local_addr()?);
    let server = axum::serve(listener, app.router.into_make_service());
    Ok(server.await?)
}
