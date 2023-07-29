mod recipe;

use std::net::TcpListener;

use backend::app::App;
use sqlx::PgPool;

struct TestApp {
    pub client: reqwest::Client,
    pub address: String,
}

impl TestApp {
    fn new(pool: PgPool) -> Result<Self, anyhow::Error> {
        // TODO: Maybe not use 0.0.0.0 per zero2prod book
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
        let address = format!("http://{}", listener.local_addr()?);
        let app = App::new(listener, pool)?;
        tokio::spawn(app.server);
        let client = reqwest::Client::new();
        Ok(Self { address, client })
    }
}
