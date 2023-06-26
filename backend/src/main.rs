use axum::{
    extract::FromRef,
    http::{HeaderValue, Method},
    routing::{get, post},
    Extension, Router,
};

use axum::http::header::CONTENT_TYPE;
use axum_login::axum_sessions::{async_session::MemoryStore, SessionLayer};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use rand::Rng;
use tower_http::cors::CorsLayer;

use crate::api::{
    auth::{google_login, login_authorized},
    user::create_user,
};

mod api;
mod services;

// type AuthContext = axum_login::extractors::AuthContext<Uuid, User, PostgresStore<User>>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let pool = sqlx::PgPool::connect(&dotenv::var("DATABASE_URL").unwrap()).await?;

    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let oauth_client = get_oauth_client();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store.clone(), &secret).with_secure(false);

    let app_state = AppState {
        store: session_store,
        oauth_client,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE])
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/google-login", get(google_login))
                .route("/authorized", get(login_authorized))
                .route("/user/create", post(create_user)),
        )
        .with_state(app_state)
        .layer(Extension(pool))
        .layer(cors)
        .layer(session_layer);

    let port = 42069;
    println!("Server running on localhost:{}", port);
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[derive(Clone)]
struct AppState {
    store: MemoryStore,
    oauth_client: BasicClient,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

fn get_oauth_client() -> BasicClient {
    let client_id = dotenv::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = dotenv::var("GOOGLE_CLIENT_SECRET").unwrap();
    let redirect_url = "http://localhost:42069/api/authorized".to_string();

    let auth_url = "https://accounts.google.com/o/oauth2/auth".to_string();
    let token_url = "https://accounts.google.com/o/oauth2/token".to_string();

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
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
