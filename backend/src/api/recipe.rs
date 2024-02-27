use crate::{
    auth_backend::AuthSession,
    entities::{ingredients, recipe_ingredients, recipes, sea_orm_active_enums},
    ApiError,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::recipe::{CreateRecipe, Recipe, RecipeIngredient};
use futures_util::StreamExt;
use sea_orm::{
    sea_query::OnConflict, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, LoaderTrait, QueryFilter, Set, StreamTrait, TransactionTrait,
};

// Creates a recipe. Dependant on that the ingredients are already created
pub async fn post_recipe(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<Json<Recipe>, ApiError> {
    let user = auth.user.unwrap();

    let created_ingredients = create_ingredients(&recipe, user.id, &db).await?;

    let tx = db.begin().await?;

    let created_recipe = recipes::Entity::insert(recipes::ActiveModel {
        id: NotSet,
        user_id: Set(user.id),
        name: Set(recipe.name),
        description: Set(recipe.description),
        instructions: Set(recipe.instructions),
        img: Set(recipe.img),
        servings: Set(recipe.servings),
        prep_time: Set(recipe.prep_time),
        baking_time: Set(recipe.baking_time),
        created_at: NotSet,
        updated_at: NotSet,
    })
    .exec_with_returning(&tx)
    .await?;

    let models = created_ingredients.into_iter().filter_map(|i| {
        let ri = recipe.ingredients.iter().find(|ri| ri.name == i.name)?;
        Some(recipe_ingredients::ActiveModel {
            recipe_id: Set(created_recipe.id),
            ingredient_id: Set(i.id),
            unit: Set(ri.unit.map(|u| u.into())),
            amount: Set(ri.amount),
        })
    });

    recipe_ingredients::Entity::insert_many(models)
        .exec(&tx)
        .await?;

    tx.commit().await?;

    let ingredients = get_recipe_ingredients(&db, created_recipe.id).await?;

    Ok(Json(Recipe {
        id: created_recipe.id,
        user_id: created_recipe.user_id,
        name: created_recipe.name,
        description: created_recipe.description,
        instructions: created_recipe.instructions,
        img: created_recipe.img,
        servings: created_recipe.servings,
        updated_at: created_recipe.updated_at,
        prep_time: created_recipe.prep_time,
        baking_time: created_recipe.baking_time,
        ingredients,
    }))
}

// Gets a recipe with an id
pub async fn get_recipe(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Recipe>, ApiError> {
    let user = auth.user.unwrap();

    let recipe_model = recipes::Entity::find_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .one(&db)
        .await?
        .ok_or(ApiError::RecordNotFound)?;

    let ingredients = get_recipe_ingredients(&db, recipe_model.id).await?;

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
pub async fn get_recipes(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<Recipe>>, ApiError> {
    let user = auth.user.unwrap();
    let recipes = recipes::Entity::find()
        .filter(recipes::Column::UserId.eq(user.id))
        .all(&db)
        .await?;

    let ingredients = recipes
        .load_many_to_many(ingredients::Entity, recipe_ingredients::Entity, &db)
        .await?;

    let ingredients_with_units = recipes.load_many(recipe_ingredients::Entity, &db).await?;

    let recipes = recipes
        .into_iter()
        .zip(
            ingredients
                .into_iter()
                .zip(ingredients_with_units.into_iter()),
        )
        .map(|r| {
            let ingredients =
                r.1 .0
                    .into_iter()
                    .zip(r.1 .1.into_iter())
                    .map(|i| RecipeIngredient {
                        ingredient_id: i.0.id,
                        ingredient_name: i.0.name,
                        unit: i.1.unit.map(|u| u.into()),
                        amount: i.1.amount,
                    })
                    .collect();

            Recipe {
                id: r.0.id,
                user_id: r.0.user_id,
                name: r.0.name,
                description: r.0.description,
                instructions: r.0.instructions,
                img: r.0.img,
                servings: r.0.servings,
                updated_at: r.0.updated_at,
                prep_time: r.0.prep_time,
                baking_time: r.0.baking_time,
                ingredients,
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(recipes))
}

pub async fn update_recipe(
    auth: AuthSession,
    Path(recipe_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<Json<Recipe>, ApiError> {
    let user = auth.user.unwrap();

    let created_ingredients = create_ingredients(&recipe, user.id, &db).await?;

    let tx = db.begin().await?;

    let updated_recipe = recipes::Entity::update(recipes::ActiveModel {
        id: Set(recipe_id),
        user_id: NotSet,
        name: Set(recipe.name),
        description: Set(recipe.description),
        instructions: Set(recipe.instructions),
        img: Set(recipe.img),
        servings: Set(recipe.servings),
        prep_time: Set(recipe.prep_time),
        baking_time: Set(recipe.baking_time),
        created_at: NotSet,
        updated_at: Set(chrono::Utc::now().into()),
    })
    .filter(recipes::Column::Id.eq(recipe_id))
    .filter(recipes::Column::UserId.eq(user.id))
    .exec(&tx)
    .await?;

    let models = created_ingredients.into_iter().filter_map(|i| {
        let ri = recipe.ingredients.iter().find(|ri| ri.name == i.name)?;
        Some(recipe_ingredients::ActiveModel {
            recipe_id: Set(updated_recipe.id),
            ingredient_id: Set(i.id),
            unit: Set(ri.unit.map(|u| u.into())),
            amount: Set(ri.amount),
        })
    });

    recipe_ingredients::Entity::delete_many()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_id))
        .exec(&tx)
        .await?;

    recipe_ingredients::Entity::insert_many(models)
        .exec(&tx)
        .await?;

    tx.commit().await?;

    let ingredients = get_recipe_ingredients(&db, recipe_id).await?;

    Ok(Json(Recipe {
        id: updated_recipe.id,
        user_id: updated_recipe.user_id,
        name: updated_recipe.name,
        description: updated_recipe.description,
        instructions: updated_recipe.instructions,
        img: updated_recipe.img,
        servings: updated_recipe.servings,
        updated_at: updated_recipe.updated_at,
        prep_time: updated_recipe.prep_time,
        baking_time: updated_recipe.baking_time,
        ingredients,
    }))
}

pub async fn delete_recipe(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
    Path(recipe_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth.user.unwrap();
    recipes::Entity::delete_by_id(recipe_id)
        .filter(recipes::Column::UserId.eq(user.id))
        .exec(&db)
        .await?;

    Ok(())
}

async fn create_ingredients(
    recipe: &CreateRecipe,
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<Vec<ingredients::Model>, anyhow::Error> {
    let ingredients: (Vec<String>, Vec<ingredients::ActiveModel>) = recipe
        .ingredients
        .iter()
        .map(|i| {
            (
                i.name.clone(),
                ingredients::ActiveModel {
                    id: NotSet,
                    name: Set(i.name.clone()),
                    user_id: Set(user_id),
                },
            )
        })
        .unzip();

    ingredients::Entity::insert_many(ingredients.1)
        .on_conflict(
            OnConflict::column(ingredients::Column::Name)
                .update_column(ingredients::Column::Name)
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(ingredients::Entity::find()
        .filter(ingredients::Column::Name.is_in(ingredients.0))
        .all(db)
        .await?)
}

// #[derive(Serialize, Deserialize)]
// pub struct RecipeImage {
//     name: String,
// }

// pub async fn get_presigned_url_for_upload<T>(
//     State(state): State<AppState<T>>,
//     Json(image): Json<RecipeImage>,
// ) -> Result<Json<RecipeImage>, ApiError>
// where
//     T: FoodieStorage + Send + Sync + Clone,
// {
//     let url = state
//         .storage
//         .get_presigned_url(&image.name, Method::PUT)
//         .await?;

//     Ok(Json(RecipeImage { name: url }))
// }

async fn get_recipe_ingredients<C>(
    db: &C,
    recipe_id: i32,
) -> Result<Vec<RecipeIngredient>, anyhow::Error>
where
    C: ConnectionTrait + Send + StreamTrait,
{
    let ingredients = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_id))
        .find_also_related(ingredients::Entity)
        .stream(db)
        .await?
        .map(|i| {
            let i = i.unwrap();
            RecipeIngredient {
                ingredient_id: i.0.ingredient_id,
                ingredient_name: i.1.unwrap().name,
                unit: i.0.unit.map(|u| u.into()),
                amount: i.0.amount,
            }
        })
        .collect::<Vec<_>>()
        .await;

    Ok(ingredients)
}

macro_rules! convert_unit {
    ($first:ty, $second: ty) => {
        impl From<$first> for $second {
            fn from(value: $first) -> Self {
                match value {
                    <$first>::Milligram => <$second>::Milligram,
                    <$first>::Gram => <$second>::Gram,
                    <$first>::Hectogram => <$second>::Hectogram,
                    <$first>::Kilogram => <$second>::Kilogram,
                    <$first>::Milliliter => <$second>::Milliliter,
                    <$first>::Deciliter => <$second>::Deciliter,
                    <$first>::Liter => <$second>::Liter,
                    <$first>::Teaspoon => <$second>::Teaspoon,
                    <$first>::Tablespoon => <$second>::Tablespoon,
                    <$first>::Cup => <$second>::Cup,
                    <$first>::Clove => <$second>::Clove,
                    <$first>::Pinch => <$second>::Pinch,
                }
            }
        }
    };
}

convert_unit!(common::recipe::Unit, sea_orm_active_enums::Unit);
convert_unit!(sea_orm_active_enums::Unit, common::recipe::Unit);
