use crate::db::FoodieDatabase;
use axum::{extract::State, Extension, Json};
use common::{ingredient::CreateIngredient, user::User};
use uuid::Uuid;

pub async fn post_ingredient(
    Extension(user): Extension<User>,
    State(db): State<FoodieDatabase>,
    Json(ingredient): Json<CreateIngredient>,
) -> Json<Uuid> {
    let created_ingredient = db
        .get(Some(user.id))
        .create_ingredient(&ingredient)
        .await
        .unwrap();
    Json(created_ingredient.id)
}
