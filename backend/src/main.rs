use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Extension, Router,
};

use axum::http::header::CONTENT_TYPE;
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use rand::Rng;
use sqlx::types::Uuid;
use tower_http::cors::{Any, CorsLayer};

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

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE])
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap());

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
        .layer(cors)
        .layer(auth_layer)
        .layer(session_layer);

    let port = 6000;
    println!("Server running on localhost:{}", port);
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use aws_sdk_s3::Client as S3Client;
    #[tokio::test(flavor = "multi_thread")]
    async fn test_upload_image() -> Result<(), anyhow::Error> {
        let client = S3Client::from_conf(
            aws_sdk_s3::config::Builder::new()
                .endpoint_url("http://localhost:4566/".to_string())
                .force_path_style(true)
                .build(),
        );

        let resp = client
            .delete_object()
            .bucket("images")
            .key("foo")
            .send()
            .await?;
        println!("resp: {:?}", resp);

        // let body = ByteStream::from_path(Path::new(
        //     "/Users/sebastianlyngjohansen/projects/foodie/sea.jpg",
        // ))
        // .await?;

        // let resp = client
        //     .put_object()
        //     .bucket("images")
        //     .key("foo")
        //     .body(body)
        //     .send()
        //     .await
        //     .unwrap();

        // println!("resp: {:?}", resp);

        // client
        //     .put_object()
        //     .bucket(bucket_name)
        //     .key("foo")
        //     .body(body)
        //     .send()
        //     .await?;

        Ok(())
    }
}
