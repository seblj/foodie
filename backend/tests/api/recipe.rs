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
    ingredient_ids: &[i32],
) -> Result<Response, anyhow::Error> {
    let ingredients: Vec<_> = ingredient_ids
        .iter()
        .map(|id| CreateRecipeIngredient {
            ingredient_id: *id,
            unit: Some(Unit::Gram),
            amount: Some(Decimal::from(10)),
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

    let response = create_recipe(&app, "Pizza", &[flour, yiest, water]).await?;

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

    let response = create_recipe(&app, "Pizza", &[flour, yiest, water]).await?;
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

    let response = create_recipe(&app, "Pizza", &[flour, yiest, water]).await?;
    let recipe_id = response.json::<i32>().await?;

    let res = app.get(format!("api/recipe/{}", recipe_id)).await?;

    let recipe = res.json::<Recipe>().await?;

    assert_eq!("Pizza", recipe.name);
    assert_eq!(Some("My pizza recipe"), recipe.description.as_deref());
    assert_eq!(None, recipe.img);

    let recipe_names = recipe
        .ingredients
        .into_iter()
        .map(|i| i.ingredient_name)
        .collect::<Vec<_>>();

    assert_eq!(["Flour", "Yiest", "Water"].to_vec(), recipe_names);

    Ok(())
}
