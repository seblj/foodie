use chrono::NaiveTime;
use common::{
    ingredient::CreateIngredient,
    recipe::{CreateRecipe, CreateRecipeIngredient, Unit},
};
use hyper::StatusCode;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::TestApp;

#[sqlx::test(migrations = "src/db/migrations")]
async fn create_and_get_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    // TODO: Should I make the requests less verbose?
    let flour: Uuid = app
        .post(
            "api/ingredient",
            &CreateIngredient {
                name: "Flour".into(),
            },
        )
        .await?
        .json()
        .await?;

    let yiest: Uuid = app
        .post(
            "api/ingredient",
            &CreateIngredient {
                name: "yiest".into(),
            },
        )
        .await?
        .json()
        .await?;

    let water: Uuid = app
        .post(
            "api/ingredient",
            &CreateIngredient {
                name: "Water".into(),
            },
        )
        .await?
        .json()
        .await?;

    let ingredients = vec![
        CreateRecipeIngredient {
            ingredient_id: flour,
            unit: Some(Unit::Tablespoon),
            amount: Some(Decimal::from(3)),
        },
        CreateRecipeIngredient {
            ingredient_id: yiest,
            unit: None,
            amount: Some(Decimal::from(1)),
        },
        CreateRecipeIngredient {
            ingredient_id: water,
            unit: Some(Unit::Deciliter),
            amount: Some(Decimal::from(6)),
        },
    ];

    let recipe = CreateRecipe {
        name: "Pizza".to_string(),
        description: None,
        img: None,
        instructions: None,
        ingredients: ingredients.clone(),
        baking_time: NaiveTime::from_hms_opt(0, 20, 0),
        prep_time: NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
        servings: 4,
    };

    let response = app.post("api/recipe", &recipe).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let recipe = app.get_recipe(response.json::<Uuid>().await?).await;
    assert_eq!("Pizza", &recipe.name);
    assert_eq!(4, recipe.servings);

    Ok(())
}
