use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use axum_login::{secrecy::SecretVec, AuthUser};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    services::user::{create_user, get_user},
    AuthContext,
};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub email: String,
    // picture: String,
}

#[derive(sqlx::FromRow, Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    // picture: String,
}

impl AuthUser<Uuid> for User {
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        SecretVec::new("".into())
    }
}

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    mut auth: AuthContext,
    State(oauth_client): State<BasicClient>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let user_info: UserInfo = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<UserInfo>()
        .await
        .unwrap();

    // TODO: This is ugly
    create_user(&pool, &user_info).await.unwrap();
    let user = get_user(&pool, user_info.email).await.unwrap();
    auth.login(&user).await.expect("Couldn't log user in");
    Redirect::to("http://localhost:8080")
}

pub async fn logout(mut auth: AuthContext) {
    auth.logout().await;
}

pub async fn foo(Extension(user): Extension<User>) -> Json<User> {
    Json(user)
}
