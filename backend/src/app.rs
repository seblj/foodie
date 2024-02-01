use crate::{
    api::{
        auth::{get_me, login, logout, register},
        ingredient::{delete_ingredient, get_ingredient, get_ingredients, post_ingredient},
        oauth::{google_callback, google_login},
        recipe::{delete_recipe, get_recipe, get_recipes, post_recipe, update_recipe},
    },
    auth_backend::{get_oauth_client, Backend},
};
use axum::{
    error_handling::HandleErrorLayer,
    http::{HeaderValue, StatusCode},
    routing::{get, post},
    Router,
};
use axum_login::{
    login_required,
    tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use hyper::{header::CONTENT_TYPE, Method};
use sea_orm::DatabaseConnection;
use std::sync::Once;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub struct App {
    pub router: Router,
    pub app_state: AppState,
    pub backend: Backend,
    pub session_layer: SessionManagerLayer<MemoryStore>,
}

static INIT: Once = Once::new();

impl App {
    pub fn new(db: DatabaseConnection) -> Result<Self, anyhow::Error> {
        INIT.call_once(|| {
            env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .filter_level(log::LevelFilter::Error)
                .init();
        });

        let oauth_client = get_oauth_client()?;

        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            // TODO: Turn on for prod
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        let backend = Backend::new(db.clone(), oauth_client);
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_| async {
                StatusCode::UNAUTHORIZED
            }))
            .layer(AuthManagerLayerBuilder::new(backend.clone(), session_layer.clone()).build());

        let app_state = AppState { db };

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_headers([CONTENT_TYPE])
            .allow_origin("http://localhost:8080".parse::<HeaderValue>()?);

        let router = Router::new()
            .nest(
                "/api",
                Router::new()
                    .route("/recipe", get(get_recipes).post(post_recipe))
                    .route(
                        "/recipe/:id",
                        get(get_recipe).delete(delete_recipe).put(update_recipe),
                    )
                    .route("/ingredient", post(post_ingredient).get(get_ingredients))
                    .route(
                        "/ingredient/:id",
                        get(get_ingredient).delete(delete_ingredient),
                    )
                    .route("/me", get(get_me))
                    .route_layer(login_required!(Backend))
                    .route("/health-check", get(|| async {}))
                    .route("/register", post(register))
                    .route("/login", post(login))
                    .route("/logout", post(logout))
                    .nest(
                        "/oauth",
                        Router::new()
                            .route("/google/login", get(google_login))
                            .route("/google/callback", get(google_callback)),
                    ),
            )
            .with_state(app_state.clone())
            .layer(auth_service)
            .layer(cors);

        Ok(Self {
            router,
            app_state,
            backend,
            session_layer,
        })
    }
}
