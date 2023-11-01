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
    let app = TestApp::new(pool).await?;

    let ingredient_id: Uuid = app
        .post("api/ingredient", &CreateIngredient { name: "Foo".into() })
        .await?
        .json()
        .await?;

    let ingredients = vec![CreateRecipeIngredient {
        ingredient_id,
        unit: Some(Unit::Tablespoon),
        amount: Some(Decimal::from(3)),
    }];

    let recipe = CreateRecipe {
        name: "Foobar".to_string(),
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

    let recipe = app
        .get_recipe(response.json::<Uuid>().await?)
        .await
        .unwrap();
    assert_eq!("Foobar", &recipe.name);
    assert_eq!(4, recipe.servings);

    Ok(())
}

#[sqlx::test(migrations = "src/db/migrations")]
async fn create_and_get_recipe_not_mine(pool: PgPool) -> Result<(), anyhow::Error> {
    let mut app = TestApp::new(pool).await?;

    let ingredient_id: Uuid = app
        .post("api/ingredient", &CreateIngredient { name: "Foo".into() })
        .await?
        .json()
        .await?;

    let ingredients = vec![CreateRecipeIngredient {
        ingredient_id,
        unit: Some(Unit::Tablespoon),
        amount: Some(Decimal::from(3)),
    }];

    let recipe = CreateRecipe {
        name: "Foobar".to_string(),
        description: None,
        img: None,
        instructions: None,
        ingredients: ingredients.clone(),
        baking_time: NaiveTime::from_hms_opt(0, 20, 0),
        prep_time: NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
        servings: 4,
    };

    let response = app.post("api/recipe", &recipe).await?;
    let status = response.status();
    let recipe_id = response.json::<Uuid>().await?;

    assert_eq!(status, StatusCode::OK);

    let recipe = app.get_recipe(recipe_id).await.unwrap();
    assert_eq!("Foobar", &recipe.name);
    assert_eq!(4, recipe.servings);

    app.set_user("bar@bar.com").await?;

    let recipe = app.get_recipe(recipe_id).await;
    println!("recipe: {:#?}", recipe);

    assert!(recipe.is_none());

    Ok(())
}

// #[sqlx::test(migrations = "src/db/migrations")]
// async fn cannot_get_others_ingredient(pool: PgPool) -> Result<(), anyhow::Error> {
//     let mut app = TestApp::new(pool).await?;

//     let response = app
//         .post("api/ingredient", &CreateIngredient { name: "Foo".into() })
//         .await?;

//     let status = response.status();
//     let ingredient_id = response.json::<Uuid>().await?;

//     assert_eq!(status, StatusCode::OK);

//     let ingredient = app.get_ingredient(ingredient_id).await.unwrap();
//     assert_eq!("Foo", &ingredient.name);

//     app.set_user("bar@bar.com").await?;

//     let ingredient = app.get_recipe(ingredient_id).await;

//     assert!(ingredient.is_none());

//     Ok(())
// }
