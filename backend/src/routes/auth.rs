use crate::db::FoodieDatabase;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use common::user::CreateUser;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use serde::Deserialize;

use crate::app::AuthContext;

pub async fn logout(mut auth: AuthContext) {
    auth.logout().await;
}

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
    mut auth: AuthContext,
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

    let user = db.get(None).create_user(&user_info).await.unwrap();
    auth.login(&user).await.expect("Couldn't log user in");

    Redirect::to("http://localhost:8080")
}
