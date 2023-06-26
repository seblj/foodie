use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use axum_login::axum_sessions::async_session::MemoryStore;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    name: String,
    email: String,
    picture: String,
}

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(store): State<MemoryStore>,
    State(oauth_client): State<BasicClient>,
) -> impl IntoResponse {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let user_data: UserInfo = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<UserInfo>()
        .await
        .unwrap();

    Json(user_data)
}

// pub async fn login(
//     mut auth: AuthContext,
//     Extension(pool): Extension<PgPool>,
//     Json(login_info): Json<LoginInfo>,
// ) -> (StatusCode, String) {
//     match auth::authenticate(pool, login_info).await {
//         Ok(user) => {
//             auth.login(&user).await.expect("Couldn't login user");
//             (StatusCode::OK, "Authorized".to_string())
//         }
//         Err(_) => (StatusCode::BAD_REQUEST, "Not authorized".to_string()),
//     }
// }

// pub async fn logout(mut auth: AuthContext) {
//     auth.logout().await;
// }
