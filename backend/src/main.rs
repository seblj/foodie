use api::auth::User;
use axum::{
    extract::FromRef,
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};

use axum::http::header::CONTENT_TYPE;
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use rand::Rng;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use crate::api::auth::{foo, google_login, login_authorized, logout, user_info};

mod api;
mod services;

type AuthContext = axum_login::extractors::AuthContext<Uuid, User, PostgresStore<User>>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let pool = sqlx::PgPool::connect(&dotenv::var("DATABASE_URL")?).await?;

    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let oauth_client = get_oauth_client()?;

    let user_store = PostgresStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store.clone(), &secret).with_secure(false);

    let app_state = AppState { oauth_client, pool };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE])
        .allow_origin("http://localhost:8080".parse::<HeaderValue>()?);

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/foo", get(foo))
                .route("/me", get(user_info))
                .route_layer(RequireAuthorizationLayer::<Uuid, User>::login())
                .route("/google-login", get(google_login))
                .route("/logout", post(logout))
                .route("/authorized", get(login_authorized)),
        )
        .with_state(app_state)
        .layer(cors)
        .layer(auth_layer)
        .layer(session_layer);

    let port = 42069;
    println!("Server running on localhost:{}", port);
    axum::Server::bind(&format!("0.0.0.0:{port}").parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[derive(Clone)]
struct AppState {
    oauth_client: BasicClient,
    pool: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
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
