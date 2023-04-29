use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use rand::Rng;
use sqlx::types::Uuid;

use crate::{
    api::{
        auth::{bar, foo, login, logout},
        user::create_user,
    },
    db::connect,
    services::auth::User,
};

mod api;
mod db;
mod services;

type AuthContext = axum_login::extractors::AuthContext<Uuid, User, PostgresStore<User>>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let pool = connect().await?;

    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let user_store = PostgresStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    sqlx::migrate!().run(&pool).await?;

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/foo", get(foo))
                .route("/bar", get(bar))
                .route_layer(RequireAuthorizationLayer::<Uuid, User>::login())
                .route("/login", post(login))
                .route("/logout", post(logout))
                .route("/user/create", post(create_user)),
        )
        .layer(Extension(pool))
        .layer(auth_layer)
        .layer(session_layer);

    let port = 6000;
    println!("Server running on localhost:{}", port);
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
