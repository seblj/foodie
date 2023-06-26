use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::services::user;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(create_user_info): Json<CreateUser>,
) -> StatusCode {
    match user::create_user(pool, create_user_info).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
