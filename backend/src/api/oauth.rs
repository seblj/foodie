use crate::auth_backend::{AuthSession, Credentials};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};
use axum_login::tower_sessions::Session;
use hyper::StatusCode;
use serde::Deserialize;

pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn google_callback(
    Query(query): Query<AuthRequest>,
    mut auth: AuthSession,
) -> impl IntoResponse {
    let user = match auth.authenticate(Credentials::new(query.code)).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("http://localhost:8080").into_response()
}

pub async fn google_login(auth: AuthSession, session: Session) -> impl IntoResponse {
    let (auth_url, csrf_state) = auth.backend.authorize_url();
    session
        .insert(CSRF_STATE_KEY, csrf_state.secret())
        .expect("Serialization should not fail.");

    Redirect::to(auth_url.as_str()).into_response()
}
