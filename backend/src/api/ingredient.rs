use crate::{app::AppState, entities::ingredients, ApiError};
use axum::{extract::State, Json};
use common::ingredient::CreateIngredient;
use sea_orm::{sea_query::OnConflict, EntityTrait, Set};

pub async fn post_ingredient(
    State(state): State<AppState>,
    Json(ingredient): Json<CreateIngredient>,
) -> Result<Json<i32>, ApiError> {
    let created_ingredient = ingredients::Entity::insert(ingredients::ActiveModel {
        name: Set(ingredient.name),
        ..Default::default()
    })
    .on_conflict(
        OnConflict::column(ingredients::Column::Name)
            .update_column(ingredients::Column::Name)
            .to_owned(),
    )
    .exec_with_returning(&state.db)
    .await?;

    Ok(Json(created_ingredient.id))
}
