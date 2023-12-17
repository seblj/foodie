use crate::{
    app::AppState,
    auth_backend::AuthSession,
    entities::{
        ingredients, recipe_ingredients, recipes,
        sea_orm_active_enums::{self},
    },
    ApiError,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::recipe::{CreateRecipe, Recipe, RecipeIngredient};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

// NOTE: The body must always be the last extractor
// Creates a recipe. Dependant on that the ingredients are already created
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

// Gets a recipe with an id
pub async fn get_recipe(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Recipe>, ApiError> {
    let user = auth.user.unwrap();

    let recipe_model = recipes::Entity::find_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .one(&state.db)
        .await?
        .unwrap();

    // TODO: Should I map with stream here?
    let ingredients_with_units = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_model.id))
        .find_also_related(ingredients::Entity)
        .all(&state.db)
        .await?;

    let ingredients = ingredients_with_units
        .into_iter()
        .map(|i| RecipeIngredient {
            ingredient_id: i.0.ingredient_id,
            ingredient_name: i.1.unwrap().name,
            unit: i.0.unit.map(|u| u.into()),
            amount: i.0.amount,
        })
        .collect();

    Ok(Json(Recipe {
        id: recipe_model.id,
        user_id: recipe_model.user_id,
        name: recipe_model.name,
        description: recipe_model.description,
        instructions: recipe_model.instructions,
        img: recipe_model.img,
        servings: recipe_model.servings,
        updated_at: recipe_model.updated_at,
        prep_time: recipe_model.prep_time,
        baking_time: recipe_model.baking_time,
        ingredients,
    }))
}

// Gets all the recipes for the user
// pub async fn get_recipes(
//     auth: AuthSession,
//     State(state): State<AppState>,
// ) -> Result<Json<Recipe>, ApiError> {
//     let user = auth.user.unwrap();

//     let recipe_model = recipes::Entity::find()
//         .filter(recipes::Column::UserId.eq(user.id))
//         .all(&state.db)
//         .await?;

//     // TODO: Should I map with stream here?
//     let ingredients_with_units = recipe_ingredients::Entity::find()
//         .filter(recipe_ingredients::Column::RecipeId.eq(recipe_model.id))
//         .find_also_related(ingredients::Entity)
//         .all(&state.db)
//         .await?;

//     let ingredients = ingredients_with_units
//         .into_iter()
//         .map(|i| RecipeIngredient {
//             ingredient_id: i.0.ingredient_id,
//             ingredient_name: i.1.unwrap().name,
//             unit: i.0.unit.map(|u| u.into()),
//             amount: i.0.amount,
//         })
//         .collect();

//     Ok(Json(Recipe {
//         id: recipe_model.id,
//         user_id: recipe_model.user_id,
//         name: recipe_model.name,
//         description: recipe_model.description,
//         instructions: recipe_model.instructions,
//         img: recipe_model.img,
//         servings: recipe_model.servings,
//         updated_at: recipe_model.updated_at,
//         prep_time: recipe_model.prep_time,
//         baking_time: recipe_model.baking_time,
//         ingredients,
//     }))
// }

pub async fn delete_recipe(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(recipe_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth.user.unwrap();
    let transaction = state.db.begin().await?;

    recipe_ingredients::Entity::delete_many()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_id))
        .exec(&transaction)
        .await?;

    recipes::Entity::delete_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .exec(&transaction)
        .await
        .unwrap();

    transaction.commit().await?;

    Ok(())
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

impl From<sea_orm_active_enums::Unit> for common::recipe::Unit {
    fn from(value: sea_orm_active_enums::Unit) -> Self {
        match value {
            sea_orm_active_enums::Unit::Milligram => common::recipe::Unit::Milligram,
            sea_orm_active_enums::Unit::Gram => common::recipe::Unit::Gram,
            sea_orm_active_enums::Unit::Hectogram => common::recipe::Unit::Hectogram,
            sea_orm_active_enums::Unit::Kilogram => common::recipe::Unit::Kilogram,
            sea_orm_active_enums::Unit::Milliliter => common::recipe::Unit::Milliliter,
            sea_orm_active_enums::Unit::Deciliter => common::recipe::Unit::Deciliter,
            sea_orm_active_enums::Unit::Liter => common::recipe::Unit::Liter,
            sea_orm_active_enums::Unit::Teaspoon => common::recipe::Unit::Teaspoon,
            sea_orm_active_enums::Unit::Tablespoon => common::recipe::Unit::Tablespoon,
            sea_orm_active_enums::Unit::Cup => common::recipe::Unit::Cup,
            sea_orm_active_enums::Unit::Clove => common::recipe::Unit::Clove,
            sea_orm_active_enums::Unit::Pinch => common::recipe::Unit::Pinch,
        }
    }
}
