use crate::db::FoodieDatabase;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use common::user::{CreateUser, User};

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::{Client, Url};
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

    Redirect::to(auth_url.as_ref())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize, Debug)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
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

    let google_user = get_google_user(token.access_token().secret())
        .await
        .unwrap();

    let create_user = CreateUser {
        name: google_user.name,
        email: google_user.email,
    };

    let user = db.create_user(&create_user).await.unwrap();
    auth.login(&user).await.expect("Couldn't log user in");

    Redirect::to("http://localhost:8080")
}

// TODO: Migrate out
pub async fn get_me(
    Extension(user): Extension<User>,
    State(db): State<FoodieDatabase>,
) -> impl IntoResponse {
    let user_info = db.get_user_by_email(&user.email).await.unwrap();
    Json(user_info)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

async fn get_google_user(access_token: &str) -> Result<GoogleUserResult, anyhow::Error> {
    let client = Client::new();
    let url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo")?;
    let response = client.get(url).bearer_auth(access_token).send().await?;

    let user_info = response.json::<GoogleUserResult>().await?;

    Ok(user_info)
}

pub fn get_oauth_client() -> Result<BasicClient, anyhow::Error> {
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
