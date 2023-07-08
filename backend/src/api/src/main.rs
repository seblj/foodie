use app::App;
use axum::extract::FromRef;

use axum_login::PostgresStore;
use common::user::User;
use db::FoodieDatabase;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use uuid::Uuid;

mod app;
mod routes;

type AuthContext = axum_login::extractors::AuthContext<Uuid, User, PostgresStore<User>>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pool = sqlx::PgPool::connect(&dotenv::var("DATABASE_URL")?).await?;
    App::new(42069, pool).await?.run().await
}

#[derive(Clone)]
struct AppState {
    oauth_client: BasicClient,
    db: FoodieDatabase,
}

impl FromRef<AppState> for FoodieDatabase {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

fn get_oauth_client() -> Result<BasicClient, anyhow::Error> {
    let client_id = dotenv::var("GOOGLE_CLIENT_ID")?;
    let client_secret = dotenv::var("GOOGLE_CLIENT_SECRET")?;
    let redirect_url = "http://localhost:42069/api/authorized".to_string();

    // access_type=offline&prompt=consent makes it return a refresh token
    let auth_url = "https://accounts.google.com/o/oauth2/auth".to_string();
    let token_url = "https://accounts.google.com/o/oauth2/token".to_string();

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?))
}
