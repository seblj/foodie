use backend::entities::recipes;
use chrono::NaiveTime;
use common::{
    ingredient::CreateIngredient,
    recipe::{CreateRecipe, CreateRecipeIngredient, Recipe, Unit},
};
use reqwest::{Response, StatusCode};
use rust_decimal::Decimal;
use sea_orm::EntityTrait;
use sqlx::PgPool;

use crate::TestApp;

async fn create_ingredient(app: &TestApp, name: &str) -> Result<i32, anyhow::Error> {
    Ok(app
        .post("api/ingredient", &CreateIngredient { name: name.into() })
        .await?
        .json::<i32>()
        .await?)
}

async fn create_recipe(
    app: &TestApp,
    name: &str,
    ingredient_ids: &[(i32, Option<Unit>, Option<Decimal>)],
) -> Result<Response, anyhow::Error> {
    let ingredients: Vec<_> = ingredient_ids
        .iter()
        .map(|i| CreateRecipeIngredient {
            ingredient_id: i.0,
            unit: i.1,
            amount: i.2,
        })
        .collect();

    let recipe = CreateRecipe {
        name: name.to_string(),
        description: Some("My pizza recipe".to_string()),
        img: None,
        instructions: None,
        ingredients: ingredients.clone(),
        baking_time: NaiveTime::from_hms_opt(0, 20, 0),
        prep_time: NaiveTime::from_hms_opt(4, 0, 0),
        servings: 4,
    };
    app.post("api/recipe", &recipe).await
}

#[sqlx::test(migrations = false)]
async fn test_create_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    let flour: i32 = create_ingredient(&app, "Flour").await?;
    let yiest: i32 = create_ingredient(&app, "Yiest").await?;
    let water: i32 = create_ingredient(&app, "Water").await?;

    let response = create_recipe(
        &app,
        "Pizza",
        &[
            (flour, Some(Unit::Kilogram), Some(Decimal::from(1))),
            (yiest, Some(Unit::Gram), Some(Decimal::from(20))),
            (water, Some(Unit::Deciliter), Some(Decimal::from(6))),
        ],
    )
    .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let recipe_id = response.json::<i32>().await?;

    let recipe = recipes::Entity::find_by_id(recipe_id)
        .one(&app.pool)
        .await?
        .unwrap();

    assert_eq!("Pizza", &recipe.name);
    assert_eq!(4, recipe.servings);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_delete_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    let flour: i32 = create_ingredient(&app, "Flour").await?;
    let yiest: i32 = create_ingredient(&app, "Yiest").await?;
    let water: i32 = create_ingredient(&app, "Water").await?;

    let response = create_recipe(
        &app,
        "Pizza",
        &[
            (flour, Some(Unit::Kilogram), Some(Decimal::from(1))),
            (yiest, Some(Unit::Gram), Some(Decimal::from(20))),
            (water, Some(Unit::Deciliter), Some(Decimal::from(6))),
        ],
    )
    .await?;
    let recipe_id = response.json::<i32>().await?;

    app.delete(format!("api/recipe/{}", recipe_id)).await?;

    let recipe = recipes::Entity::find_by_id(recipe_id)
        .one(&app.pool)
        .await?;

    assert_eq!(None, recipe);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_get_recipe_by_id(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    let flour: i32 = create_ingredient(&app, "Flour").await?;
    let yiest: i32 = create_ingredient(&app, "Yiest").await?;
    let water: i32 = create_ingredient(&app, "Water").await?;

    let response = create_recipe(
        &app,
        "Pizza",
        &[
            (flour, Some(Unit::Kilogram), Some(Decimal::from(1))),
            (yiest, Some(Unit::Gram), Some(Decimal::from(20))),
            (water, Some(Unit::Deciliter), Some(Decimal::from(6))),
        ],
    )
    .await?;
    let recipe_id = response.json::<i32>().await?;

    let res = app.get(format!("api/recipe/{}", recipe_id)).await?;

    let recipe = res.json::<Recipe>().await?;

    assert_eq!("Pizza", recipe.name);
    assert_eq!(Some("My pizza recipe"), recipe.description.as_deref());
    assert_eq!(None, recipe.img);

    let ingredient_names = recipe
        .ingredients
        .into_iter()
        .map(|i| i.ingredient_name)
        .collect::<Vec<_>>();

    assert_eq!(["Flour", "Yiest", "Water"].to_vec(), ingredient_names);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_get_all_recipes(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;

    let flour: i32 = create_ingredient(&app, "Flour").await?;
    let yiest: i32 = create_ingredient(&app, "Yiest").await?;
    let water: i32 = create_ingredient(&app, "Water").await?;

    let egg: i32 = create_ingredient(&app, "Egg").await?;
    let milk: i32 = create_ingredient(&app, "Milk").await?;

    let bread: i32 = create_ingredient(&app, "Bread").await?;
    let cheese: i32 = create_ingredient(&app, "Cheese").await?;
    let butter: i32 = create_ingredient(&app, "Butter").await?;

    create_recipe(
        &app,
        "Pizza",
        &[
            (flour, Some(Unit::Kilogram), Some(Decimal::from(1))),
            (yiest, Some(Unit::Gram), Some(Decimal::from(20))),
            (water, Some(Unit::Deciliter), Some(Decimal::from(6))),
        ],
    )
    .await?;

    create_recipe(
        &app,
        "Pancakes",
        &[
            (flour, Some(Unit::Kilogram), Some(Decimal::from(1))),
            (milk, Some(Unit::Cup), Some(Decimal::from(1))),
            (egg, None, Some(Decimal::from(1))),
        ],
    )
    .await?;

    create_recipe(
        &app,
        "Toast",
        &[
            (bread, None, Some(Decimal::from(2))),
            (cheese, Some(Unit::Gram), Some(Decimal::from(60))),
            (butter, None, None),
        ],
    )
    .await?;

    let res = app.get("api/recipe").await?;
    let recipes = res.json::<Vec<Recipe>>().await?;

    let ingredient_names = recipes[0]
        .ingredients
        .iter()
        .map(|i| (i.ingredient_name.as_str(), i.unit, i.amount))
        .collect::<Vec<_>>();

    assert_eq!(
        [
            ("Flour", Some(Unit::Kilogram), Some(Decimal::from(1))),
            ("Yiest", Some(Unit::Gram), Some(Decimal::from(20))),
            ("Water", Some(Unit::Deciliter), Some(Decimal::from(6))),
        ]
        .to_vec(),
        ingredient_names
    );

    let ingredient_names = recipes[1]
        .ingredients
        .iter()
        .map(|i| (i.ingredient_name.as_str(), i.unit, i.amount))
        .collect::<Vec<_>>();

    assert_eq!(
        [
            ("Flour", Some(Unit::Kilogram), Some(Decimal::from(1))),
            ("Milk", Some(Unit::Cup), Some(Decimal::from(1))),
            ("Egg", None, Some(Decimal::from(1))),
        ]
        .to_vec(),
        ingredient_names
    );

    let ingredient_names = recipes[2]
        .ingredients
        .iter()
        .map(|i| (i.ingredient_name.as_str(), i.unit, i.amount))
        .collect::<Vec<_>>();

    assert_eq!(
        [
            ("Bread", None, Some(Decimal::from(2))),
            ("Cheese", Some(Unit::Gram), Some(Decimal::from(60))),
            ("Butter", None, None),
        ]
        .to_vec(),
        ingredient_names
    );

    assert_eq!(3, recipes.len());

    Ok(())
}
