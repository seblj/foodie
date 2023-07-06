use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    // picture: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    // picture: String,
}

#[cfg(feature = "backend")]
impl axum_login::AuthUser<Uuid> for User {
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        axum_login::secrecy::SecretVec::new("".into())
    }
}
