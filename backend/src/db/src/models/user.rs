use axum_login::{secrecy::SecretVec, AuthUser};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUser {
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
