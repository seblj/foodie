use backend::entities::{ingredients, recipe_ingredients, recipes};
use chrono::NaiveTime;
use common::{
    ingredient::{CreateIngredient, Ingredient},
    recipe::{CreateRecipe, CreateRecipeIngredient, Recipe, Unit},
};
use reqwest::StatusCode;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sqlx::PgPool;

use crate::TestApp;

async fn create_ingredient(app: &TestApp, name: &str) -> Result<Ingredient, anyhow::Error> {
    Ok(app
        .post("api/ingredient", &CreateIngredient { name: name.into() })
        .await?
        .json::<Ingredient>()
        .await?)
}

async fn get_pizza_recipe(app: &TestApp) -> Result<CreateRecipe, anyhow::Error> {
    let flour = create_ingredient(app, "Flour").await?;
    let yiest = create_ingredient(app, "Yiest").await?;
    let water = create_ingredient(app, "Water").await?;

    let ingredients = [
        (flour.id, Some(Unit::Kilogram), Some(Decimal::from(1))),
        (yiest.id, Some(Unit::Gram), Some(Decimal::from(20))),
        (water.id, Some(Unit::Deciliter), Some(Decimal::from(6))),
    ]
    .map(|i| CreateRecipeIngredient {
        ingredient_id: i.0,
        unit: i.1,
        amount: i.2,
    });

    Ok(CreateRecipe {
        name: "My pizza".to_string(),
        description: Some("My pizza recipe".to_string()),
        img: None,
        instructions: None,
        ingredients: ingredients.to_vec(),
        baking_time: NaiveTime::from_hms_opt(0, 20, 0),
        prep_time: NaiveTime::from_hms_opt(4, 0, 0),
        servings: 4,
    })
}

async fn get_pancake_recipe(app: &TestApp) -> Result<CreateRecipe, anyhow::Error> {
    let flour = create_ingredient(app, "Flour").await?;
    let milk = create_ingredient(app, "Milk").await?;
    let egg = create_ingredient(app, "Egg").await?;

    let ingredients = [
        (flour.id, Some(Unit::Kilogram), Some(Decimal::from(1))),
        (milk.id, Some(Unit::Cup), Some(Decimal::from(1))),
        (egg.id, None, Some(Decimal::from(1))),
    ]
    .map(|i| CreateRecipeIngredient {
        ingredient_id: i.0,
        unit: i.1,
        amount: i.2,
    });

    Ok(CreateRecipe {
        name: "My pancakes".to_string(),
        description: Some("My panckace recipe".to_string()),
        img: None,
        instructions: None,
        ingredients: ingredients.to_vec(),
        baking_time: NaiveTime::from_hms_opt(0, 10, 0),
        prep_time: NaiveTime::from_hms_opt(1, 0, 0),
        servings: 4,
    })
}

async fn get_toast_recipe(app: &TestApp) -> Result<CreateRecipe, anyhow::Error> {
    let bread = create_ingredient(app, "Bread").await?;
    let cheese = create_ingredient(app, "Cheese").await?;
    let butter = create_ingredient(app, "Butter").await?;

    let ingredients = [
        (bread.id, None, Some(Decimal::from(2))),
        (cheese.id, Some(Unit::Gram), Some(Decimal::from(100))),
        (butter.id, None, None),
    ]
    .map(|i| CreateRecipeIngredient {
        ingredient_id: i.0,
        unit: i.1,
        amount: i.2,
    });

    Ok(CreateRecipe {
        name: "Toast".to_string(),
        description: Some("My toast recipe".to_string()),
        img: None,
        instructions: None,
        ingredients: ingredients.to_vec(),
        baking_time: NaiveTime::from_hms_opt(0, 11, 0),
        prep_time: NaiveTime::from_hms_opt(0, 10, 0),
        servings: 2,
    })
}

#[sqlx::test(migrations = false)]
async fn test_create_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    let pizza_recipe = get_pizza_recipe(&app).await?;
    let response = app.post("api/recipe", &pizza_recipe).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let recipe = response.json::<Recipe>().await?;

    let recipe = recipes::Entity::find_by_id(recipe.id)
        .one(&app.pool)
        .await?
        .unwrap();

    assert_eq!("My pizza", &recipe.name);
    assert_eq!(4, recipe.servings);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_delete_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    let pizza_recipe = get_pizza_recipe(&app).await?;
    let response = app.post("api/recipe", &pizza_recipe).await?;
    let recipe = response.json::<Recipe>().await?;
    let recipe_id = recipe.id;

    app.delete(format!("api/recipe/{}", recipe_id)).await?;

    let recipe = recipes::Entity::find_by_id(recipe_id)
        .one(&app.pool)
        .await?;

    let recipe_ingredients = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_id))
        .one(&app.pool)
        .await?;

    assert_eq!(None, recipe);
    assert_eq!(None, recipe_ingredients);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_get_recipe_by_id(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    let pizza_recipe = get_pizza_recipe(&app).await?;
    let response = app.post("api/recipe", &pizza_recipe).await?;
    let recipe = response.json::<Recipe>().await?;
    let recipe_id = recipe.id;

    let res = app.get(format!("api/recipe/{}", recipe_id)).await?;

    let recipe = res.json::<Recipe>().await?;

    assert_eq!("My pizza", recipe.name);
    assert_eq!(Some("My pizza recipe"), recipe.description.as_deref());
    assert_eq!(None, recipe.img);

    let ingredients = recipe
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
        ingredients
    );

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_get_all_recipes(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;

    let pizza_recipe = get_pizza_recipe(&app).await?;
    let pancake_recipe = get_pancake_recipe(&app).await?;
    let toast_recipe = get_toast_recipe(&app).await?;
    app.post("api/recipe", &pizza_recipe).await?;
    app.post("api/recipe", &pancake_recipe).await?;
    app.post("api/recipe", &toast_recipe).await?;

    let res = app.get("api/recipe").await?;
    let recipes = res.json::<Vec<Recipe>>().await?;

    let get_ingredients = |i: usize| {
        recipes[i]
            .ingredients
            .iter()
            .map(|i| (i.ingredient_name.as_str(), i.unit, i.amount))
            .collect::<Vec<_>>()
    };

    assert_eq!(
        [
            ("Flour", Some(Unit::Kilogram), Some(Decimal::from(1))),
            ("Yiest", Some(Unit::Gram), Some(Decimal::from(20))),
            ("Water", Some(Unit::Deciliter), Some(Decimal::from(6))),
        ]
        .to_vec(),
        get_ingredients(0)
    );

    assert_eq!(
        [
            ("Flour", Some(Unit::Kilogram), Some(Decimal::from(1))),
            ("Milk", Some(Unit::Cup), Some(Decimal::from(1))),
            ("Egg", None, Some(Decimal::from(1))),
        ]
        .to_vec(),
        get_ingredients(1)
    );

    assert_eq!(
        [
            ("Bread", None, Some(Decimal::from(2))),
            ("Cheese", Some(Unit::Gram), Some(Decimal::from(100))),
            ("Butter", None, None),
        ]
        .to_vec(),
        get_ingredients(2)
    );

    assert_eq!(3, recipes.len());

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_update_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;
    let pizza_recipe = get_pizza_recipe(&app).await?;

    let recipe = app
        .post("api/recipe", &pizza_recipe)
        .await?
        .json::<Recipe>()
        .await?;
    let recipe_id = recipe.id;

    let toast_recipe = get_toast_recipe(&app).await?;

    let updated_recipe = app
        .put(format!("api/recipe/{recipe_id}"), &toast_recipe)
        .await
        .unwrap();
    let updated_recipe = updated_recipe.json::<Recipe>().await?;

    assert_eq!(&updated_recipe.name, "Toast");
    assert_eq!(
        updated_recipe.description,
        Some("My toast recipe".to_string())
    );

    let recipe = recipes::Entity::find_by_id(recipe_id)
        .one(&app.pool)
        .await?
        .unwrap();

    let recipe_ingredients = recipe_ingredients::Entity::find()
        .filter(recipe_ingredients::Column::RecipeId.eq(recipe_id))
        .find_also_related(ingredients::Entity)
        .all(&app.pool)
        .await?;

    let ingredient_names = recipe_ingredients
        .iter()
        .map(|i| {
            (
                i.1.as_ref().unwrap().name.as_str(),
                i.0.unit.as_ref().map(|u| u.clone().into()),
                i.0.amount,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(&recipe.name, "Toast");
    assert_eq!(recipe.description, Some("My toast recipe".to_string()));
    assert_eq!(recipe.servings, 2);
    assert_eq!(
        [
            ("Bread", None, Some(Decimal::from(2))),
            ("Cheese", Some(Unit::Gram), Some(Decimal::from(100))),
            ("Butter", None, None),
        ]
        .to_vec(),
        ingredient_names
    );

    Ok(())
}
