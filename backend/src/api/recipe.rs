use crate::{
    app::AppState,
    auth_backend::AuthSession,
    entities::{
        recipe_ingredients, recipes,
        sea_orm_active_enums::{self},
    },
    ApiError,
};
use axum::{
    extract::{Path, State},
    Json,
};
use common::recipe::{CreateRecipe, Recipe};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

// NOTE: The body must always be the last extractor
pub async fn post_recipe(
    auth: AuthSession,
    State(state): State<AppState>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<Json<i32>, ApiError> {
    let user = auth.user.unwrap();
    let created_recipe = recipes::Entity::insert(recipes::ActiveModel {
        user_id: Set(user.id),
        name: Set(recipe.name),
        description: Set(recipe.description),
        instructions: Set(recipe.instructions),
        img: Set(recipe.img),
        servings: Set(recipe.servings),
        prep_time: Set(recipe.prep_time),
        baking_time: Set(recipe.baking_time),
        ..Default::default()
    })
    .exec_with_returning(&state.db)
    .await?;

    let models = recipe
        .ingredients
        .iter()
        .map(|i| recipe_ingredients::ActiveModel {
            recipe_id: Set(created_recipe.id),
            ingredient_id: Set(i.ingredient_id),
            unit: match i.unit {
                Some(unit) => Set(Some(unit.into())),
                None => Set(None),
            },
            amount: Set(i.amount),
            ..Default::default()
        });

    recipe_ingredients::Entity::insert_many(models)
        .exec(&state.db)
        .await?;

    Ok(Json(created_recipe.id))
}

pub async fn get_recipe(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Recipe>, ApiError> {
    let user = auth.user.unwrap();
    // TODO: I need to find a way to get the ingredient name as well
    // I don't want to do multiple database queries, but I don't know if
    // it is possible to do with sea-orm without doing multiple
    let recipe_model = recipes::Entity::find_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .find_with_related(recipe_ingredients::Entity)
        .all(&state.db)
        .await?
        .pop()
        .unwrap();

    todo!()
}

// TODO: Figure out where I should have this and if I should have it
impl From<common::recipe::Unit> for sea_orm_active_enums::Unit {
    fn from(value: common::recipe::Unit) -> Self {
        match value {
            common::recipe::Unit::Milligram => sea_orm_active_enums::Unit::Milligram,
            common::recipe::Unit::Gram => sea_orm_active_enums::Unit::Gram,
            common::recipe::Unit::Hectogram => sea_orm_active_enums::Unit::Hectogram,
            common::recipe::Unit::Kilogram => sea_orm_active_enums::Unit::Kilogram,
            common::recipe::Unit::Milliliter => sea_orm_active_enums::Unit::Milliliter,
            common::recipe::Unit::Deciliter => sea_orm_active_enums::Unit::Deciliter,
            common::recipe::Unit::Liter => sea_orm_active_enums::Unit::Liter,
            common::recipe::Unit::Teaspoon => sea_orm_active_enums::Unit::Teaspoon,
            common::recipe::Unit::Tablespoon => sea_orm_active_enums::Unit::Tablespoon,
            common::recipe::Unit::Cup => sea_orm_active_enums::Unit::Cup,
            common::recipe::Unit::Clove => sea_orm_active_enums::Unit::Clove,
            common::recipe::Unit::Pinch => sea_orm_active_enums::Unit::Pinch,
        }
    }
}
