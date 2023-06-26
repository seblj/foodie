use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;

use crate::services::user;

use super::auth::UserInfo;

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(create_user_info): Json<UserInfo>,
) -> StatusCode {
    match user::create_user(&pool, &create_user_info).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
