use std::sync::Once;

use axum::{extract::FromRef, http::HeaderValue, routing::get, routing::post, Router};

use super::db::FoodieDatabase;
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore, RequireAuthorizationLayer, SqlxStore,
};
use common::user::User;
use oauth2::basic::BasicClient;
use rand::Rng;
use reqwest::{header::CONTENT_TYPE, Method};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use crate::{
    oauth,
    routes::{
        auth::{google_login, login_authorized, logout},
        ingredient::post_ingredient,
        recipe::post_recipe,
    },
};

pub type AuthContext = axum_login::extractors::AuthContext<Uuid, User, PostgresStore<User>>;

#[derive(Clone)]
pub struct AppState {
    oauth_client: BasicClient,
    pub db: FoodieDatabase,
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

pub struct App {
    pub router: Router,
    pub app_state: AppState,
    pub auth_layer: AuthLayer<SqlxStore<PgPool, User>, Uuid, User>,
    pub session_layer: SessionLayer<MemoryStore>,
}

static INIT: Once = Once::new();

impl App {
    pub fn new(pool: PgPool) -> Result<Self, anyhow::Error> {
        INIT.call_once(|| {
            env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .filter_level(log::LevelFilter::Trace)
                .init();
        });

        let db = FoodieDatabase::new(pool.clone());

        let secret = rand::thread_rng().gen::<[u8; 64]>();
        let oauth_client = oauth::get_oauth_client()?;

        let app_state = AppState { oauth_client, db };

        let user_store = PostgresStore::<User>::new(pool);
        let auth_layer = AuthLayer::new(user_store, &secret);

        let session_store = MemoryStore::new();
        let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_headers([CONTENT_TYPE])
            .allow_origin("http://localhost:8080".parse::<HeaderValue>()?);

        let router = Router::new()
            .nest(
                "/api",
                Router::new()
                    .route("/recipe", post(post_recipe))
                    .route("/ingredient", post(post_ingredient))
                    .route_layer(RequireAuthorizationLayer::<Uuid, User>::login())
                    .route("/login", get(google_login))
                    .route("/logout", post(logout))
                    .route("/authorized", get(login_authorized))
                    .route("/health-check", get(|| async {})),
            )
            .with_state(app_state.clone())
            .layer(auth_layer.clone())
            .layer(session_layer.clone())
            .layer(cors);

        Ok(Self {
            router,
            app_state,
            auth_layer,
            session_layer,
        })
    }
}
