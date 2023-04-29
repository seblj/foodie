use crate::{
    services::auth::{self, LoginInfo, User},
    AuthContext,
};
use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;

pub async fn login(
    mut auth: AuthContext,
    Extension(pool): Extension<PgPool>,
    Json(login_info): Json<LoginInfo>,
) -> (StatusCode, String) {
    match auth::authenticate(pool, login_info).await {
        Ok(user) => {
            auth.login(&user).await.expect("Couldn't login user");
            (StatusCode::OK, "Authorized".to_string())
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Not authorized".to_string()),
    }
}

pub async fn foo(Extension(user): Extension<User>) -> Json<User> {
    println!("current user in foo: {:?}", user);
    Json(user)
}

pub async fn bar(Extension(user): Extension<User>) -> &'static str {
    println!("current user in bar: {:?}", user);
    "bar"
}

pub async fn logout(mut auth: AuthContext) {
    auth.logout().await;
}
