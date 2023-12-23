use crate::{
    app::AppState,
    auth_backend::AuthSession,
    entities::{ingredients, recipe_ingredients},
    ApiError,
};
use axum::{
    extract::{Path, State},
    Json,
};
use common::ingredient::{CreateIngredient, Ingredient};
use sea_orm::{
    sea_query::OnConflict, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set,
};

pub async fn post_ingredient(
    State(state): State<AppState>,
    auth: AuthSession,
    Json(ingredient): Json<CreateIngredient>,
) -> Result<Json<i32>, ApiError> {
    let user = auth.user.unwrap();
    let created_ingredient = ingredients::Entity::insert(ingredients::ActiveModel {
        id: NotSet,
        name: Set(ingredient.name),
        user_id: Set(user.id),
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
    Path(ingredient_id): Path<i32>,
) -> Result<Json<i32>, ApiError> {
    let recipe_ingredients = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::IngredientId.eq(ingredient_id))
        .one(&state.db)
        .await?;

    if recipe_ingredients.is_none() {
        ingredients::Entity::delete_by_id(ingredient_id)
            .exec(&state.db)
            .await?;
        Ok(Json(ingredient_id))
    } else {
        Err(ApiError::ConflictError(
            "Ingredient is a part of a recipe".to_string(),
        ))
    }
}

pub async fn get_ingredient(
    State(state): State<AppState>,
    auth: AuthSession,
    Path(ingredient_id): Path<i32>,
) -> Result<Json<Ingredient>, ApiError> {
    let user = auth.user.unwrap();
    let ingredient = ingredients::Entity::find_by_id(ingredient_id)
        .filter(ingredients::Column::UserId.eq(user.id))
        .one(&state.db)
        .await?
        .ok_or(ApiError::RecordNotFound)?;

    Ok(Json(Ingredient {
        id: ingredient.id,
        name: ingredient.name,
    }))
}
