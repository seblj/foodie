use crate::db::FoodieDatabase;
use axum::{extract::State, Json};
use common::ingredient::CreateIngredient;
use uuid::Uuid;

pub async fn post_ingredient(
    State(db): State<FoodieDatabase>,
    Json(ingredient): Json<CreateIngredient>,
) -> Json<Uuid> {
    let created_ingredient = db.create_ingredient(&ingredient).await.unwrap();
    Json(created_ingredient.id)
}
