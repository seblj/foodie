use crate::entities::users::Entity as UserEntity;
use crate::{app::AppState, auth_backend::AuthSession, entities::users, ApiError};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::response::IntoResponse;
use axum::{extract::State, Json};
use common::user::{CreateUser, User, UserLogin};
use hyper::StatusCode;
use rand::rngs::OsRng;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::NotSet;
use sea_orm::Set;

pub fn compute_hash(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .unwrap()
        .to_string()
}

pub async fn register(
    State(state): State<AppState>,
    Json(create_user): Json<CreateUser>,
) -> Result<impl IntoResponse, ApiError> {
    users::Entity::insert(users::ActiveModel {
        id: NotSet,
        email: Set(create_user.email),
        password: Set(Some(compute_hash(create_user.password.as_bytes()))),
        name: Set(create_user.name),
    })
    .on_conflict(
        OnConflict::column(users::Column::Email)
            .update_column(users::Column::Password)
            .to_owned(),
    )
    .exec(&state.db)
    .await?;

    Ok(().into_response())
}

pub async fn login(
    State(state): State<AppState>,
    mut auth: AuthSession,
    Json(user_login): Json<UserLogin>,
) -> Result<impl IntoResponse, ApiError> {
    let user_model = users::Entity::find()
        .filter(users::Column::Email.contains(user_login.email))
        .one(&state.db)
        .await?
        .unwrap();

    match user_model.password {
        Some(password) => {
            let hash = PasswordHash::new(&password).unwrap();
            if Argon2::default()
                .verify_password(user_login.password.as_bytes(), &hash)
                .is_ok()
            {
                auth.login(&User {
                    id: user_model.id,
                    name: user_model.name,
                    email: user_model.email,
                })
                .await
                .unwrap();
            } else {
                return Ok(StatusCode::UNAUTHORIZED.into_response());
            }
        }
        None => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
    Ok(().into_response())
}

pub async fn logout(mut auth: AuthSession) {
    auth.logout().await.unwrap();
}

// TODO: Migrate out
pub async fn get_me(
    auth: AuthSession,
    State(state): State<AppState>,
) -> Result<Json<User>, ApiError> {
    let user = auth.user.unwrap();
    let user_model = UserEntity::find()
        .filter(users::Column::Email.contains(user.email))
        .one(&state.db)
        .await?
        .ok_or(ApiError::RecordNotFound)?;

    let user = User {
        id: user_model.id,
        name: user_model.name,
        email: user_model.email,
    };

    Ok(Json(user))
}
