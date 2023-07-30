use std::net::TcpListener;

use axum::{
    extract::FromRef,
    http::HeaderValue,
    routing::get,
    routing::{post, IntoMakeService},
    Router,
};
use common::user::User;
use db::FoodieDatabase;
use hyper::{server::conn::AddrIncoming, Server};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use rand::Rng;
use reqwest::{header::CONTENT_TYPE, Method};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use crate::routes::{
    auth::{foo, google_login, login_authorized, user_info},
    health_check,
    recipe::post_recipe,
};

type AxumServer = Server<AddrIncoming, IntoMakeService<Router>>;

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

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone)]
enum UserRole {
    Admin,
    User,
}

pub struct App {
    pub server: AxumServer,
}

impl App {
    pub fn new(listener: TcpListener, pool: PgPool) -> Result<Self, anyhow::Error> {
        env_logger::builder()
            .format_timestamp(None)
            .filter_level(log::LevelFilter::Info)
            .init();

        let db = FoodieDatabase::new(pool.clone());

        let secret = rand::thread_rng().gen::<[u8; 64]>();
        let oauth_client = get_oauth_client()?;

        let app_state = AppState { oauth_client, db };

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_headers([CONTENT_TYPE])
            .allow_origin("http://localhost:8080".parse::<HeaderValue>()?);

        let router = Router::new()
            .nest(
                "/api",
                Router::new()
                    .route("/foo", get(foo))
                    .route("/me", get(user_info))
                    .route("/recipe", post(post_recipe))
                    // .route_layer(RequireAuthorizationLayer::<Uuid, User>::login())
                    .route("/google-login", get(google_login))
                    // .route("/authorized", get(login_authorized))
                    .route("/health-check", get(health_check)),
            )
            .with_state(app_state)
            .layer(cors);

        let server = axum::Server::from_tcp(listener)?.serve(router.into_make_service());
        println!("Server running on {}", server.local_addr());
        Ok(Self { server })
    }
}
