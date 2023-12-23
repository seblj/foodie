use backend::entities::ingredients;
use chrono::NaiveTime;
use common::{
    ingredient::CreateIngredient,
    recipe::{CreateRecipe, CreateRecipeIngredient, Unit},
};
use reqwest::Response;
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

    create_recipe(&app, "Pizza", &[flour, yiest]).await?;

    app.delete(format!("api/ingredient/{}", flour)).await?;

    let flour = ingredients::Entity::find_by_id(flour)
        .one(&app.pool)
        .await?;

    assert!(flour.is_some());

    app.delete(format!("api/ingredient/{}", water)).await?;

    let water = ingredients::Entity::find_by_id(water)
        .one(&app.pool)
        .await?;

    assert!(water.is_none());

    Ok(())
}
