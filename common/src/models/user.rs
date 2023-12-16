use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    // picture: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    // picture: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[cfg(feature = "backend")]
impl axum_login::AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &[0u8; 32]
    }
}
