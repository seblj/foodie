use axum::{http::HeaderValue, routing::get, routing::post, Router};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use common::user::User;
use db::FoodieDatabase;
use rand::Rng;
use reqwest::{header::CONTENT_TYPE, Method};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use crate::{
    get_oauth_client,
    routes::auth::{foo, google_login, login_authorized, logout, user_info},
    AppState,
};

pub struct App {
    router: Router,
    port: u16,
}

impl App {
    pub async fn new(port: u16, pool: PgPool) -> Result<Self, anyhow::Error> {
        env_logger::builder()
            .format_timestamp(None)
            .filter_level(log::LevelFilter::Info)
            .init();

        let db = FoodieDatabase::new(pool.clone());

        let secret = rand::thread_rng().gen::<[u8; 64]>();
        let oauth_client = get_oauth_client()?;

        let user_store = PostgresStore::<User>::new(pool);
        let auth_layer = AuthLayer::new(user_store, &secret);

        let session_store = MemoryStore::new();
        let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

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
                    .route_layer(RequireAuthorizationLayer::<Uuid, User>::login())
                    .route("/google-login", get(google_login))
                    .route("/logout", post(logout))
                    .route("/authorized", get(login_authorized)),
            )
            .with_state(app_state)
            .layer(cors)
            .layer(auth_layer)
            .layer(session_layer);

        Ok(Self { router, port })
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        println!("Server running on localhost:{}", self.port);
        Ok(
            axum::Server::bind(&format!("0.0.0.0:{}", self.port).parse()?)
                .serve(self.router.into_make_service())
                .await?,
        )
    }
}
