use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Extension,
};
use common::user::{CreateUser, User};
use db::FoodieDatabase;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use serde::Deserialize;

pub async fn google_login(State(client): State<BasicClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url();

    // Redirect to Google's oauth service
    Redirect::to(auth_url.as_ref())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(oauth_client): State<BasicClient>,
    State(db): State<FoodieDatabase>,
) -> impl IntoResponse {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let user_info: CreateUser = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<CreateUser>()
        .await
        .unwrap();

    let user = db.create_user(&user_info).await.unwrap();
    Redirect::to("http://localhost:8080")
}

pub async fn user_info(Extension(_): Extension<User>) {
    // TODO: Send some user info to client here and save in some state there
}
