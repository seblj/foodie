use axum::{extract::State, Extension, Json};
use common::{recipe::CreateRecipe, user::User};
use db::FoodieDatabase;
use uuid::Uuid;

// NOTE: The body must always be the last extractor
pub async fn post_recipe(
    Extension(user): Extension<User>,
    State(db): State<FoodieDatabase>,
    Json(recipe): Json<CreateRecipe>,
) -> Json<Uuid> {
    let created_recipe = db.create_recipe(user.id, &recipe).await.unwrap();
    Json(created_recipe)
}
