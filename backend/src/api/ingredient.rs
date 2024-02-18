use crate::{
    auth_backend::AuthSession,
    entities::{self, ingredients, recipe_ingredients},
    ApiError,
};
use axum::{
    extract::{Path, State},
    Json,
};
use common::ingredient::{CreateIngredient, Ingredient};
use futures_util::StreamExt;
use sea_orm::{
    sea_query::OnConflict, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};

pub async fn post_ingredient(
    State(db): State<DatabaseConnection>,
    auth: AuthSession,
    Json(ingredient): Json<CreateIngredient>,
) -> Result<Json<Ingredient>, ApiError> {
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
    .exec_with_returning(&db)
    .await?;

    Ok(Json(Ingredient {
        id: created_ingredient.id,
        name: created_ingredient.name,
    }))
}

pub async fn get_ingredients(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<Ingredient>>, ApiError> {
    let user = auth.user.unwrap();

    let ingredients = entities::ingredients::Entity::find()
        .filter(entities::ingredients::Column::UserId.eq(user.id))
        .stream(&db)
        .await?
        .map(|i| {
            // TODO: I don't want to unwrap in here maybe
            let i = i.unwrap();
            Ingredient {
                id: i.id,
                name: i.name,
            }
        })
        .collect::<Vec<_>>()
        .await;

    Ok(Json(ingredients))
}

pub async fn delete_ingredient(
    State(db): State<DatabaseConnection>,
    Path(ingredient_id): Path<i32>,
) -> Result<Json<i32>, ApiError> {
    let recipe_ingredients = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::IngredientId.eq(ingredient_id))
        .one(&db)
        .await?;

    if recipe_ingredients.is_none() {
        ingredients::Entity::delete_by_id(ingredient_id)
            .exec(&db)
            .await?;
        Ok(Json(ingredient_id))
    } else {
        Err(ApiError::ConflictError(
            "Ingredient is a part of a recipe".to_string(),
        ))
    }
}

pub async fn get_ingredient(
    State(db): State<DatabaseConnection>,
    auth: AuthSession,
    Path(ingredient_id): Path<i32>,
) -> Result<Json<Ingredient>, ApiError> {
    let user = auth.user.unwrap();
    let ingredient = ingredients::Entity::find_by_id(ingredient_id)
        .filter(ingredients::Column::UserId.eq(user.id))
        .one(&db)
        .await?
        .ok_or(ApiError::RecordNotFound)?;

    Ok(Json(Ingredient {
        id: ingredient.id,
        name: ingredient.name,
    }))
}
