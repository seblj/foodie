use crate::{app::AppState, entities::ingredients, ApiError};
use axum::{
    extract::{Path, State},
    Json,
};
use common::ingredient::CreateIngredient;
use sea_orm::{sea_query::OnConflict, EntityTrait, Set};

// TODO: Think about if I should have user id here
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

pub async fn delete_ingredient(
    State(state): State<AppState>,
    Json(ingredient): Json<CreateIngredient>,
    Path(ingredient_id): Path<i32>,
) -> Result<Json<i32>, ApiError> {
    todo!("Need to find out how I should deal with deleted ingredients for recipes");
}
