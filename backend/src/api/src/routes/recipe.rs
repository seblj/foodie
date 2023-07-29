use axum::{extract::State, Json};
use common::recipe::CreateRecipe;
use db::FoodieDatabase;
use uuid::Uuid;

// NOTE: The body must always be the last extractor
pub async fn post_recipe(
    State(db): State<FoodieDatabase>,
    Json(recipe): Json<CreateRecipe>,
) -> Json<Uuid> {
    let created_recipe = db.create_recipe(&recipe).await.unwrap();
    Json(created_recipe)
}
