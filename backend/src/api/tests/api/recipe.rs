use chrono::NaiveTime;
use common::{
    ingredient::CreateIngredient,
    recipe::{CreateRecipe, RecipeIngredient, Unit},
    user::CreateUser,
};
use db::FoodieDatabase;
use rust_decimal::Decimal;
use sqlx::PgPool;

#[sqlx::test(migrations = "../db/migrations")]
async fn create_and_get_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let db = FoodieDatabase::new(pool);

    let created_user = db
        .create_user(&CreateUser {
            name: "foo".to_string(),
            email: "foo@foo.com".to_string(),
        })
        .await?;

    let flour = db
        .create_ingredient(&CreateIngredient {
            name: "Flour".to_string(),
        })
        .await?;
    let yiest = db
        .create_ingredient(&CreateIngredient {
            name: "Yiest".to_string(),
        })
        .await?;
    let water = db
        .create_ingredient(&CreateIngredient {
            name: "Water".to_string(),
        })
        .await?;

    let ingredients = vec![
        RecipeIngredient {
            ingredient_id: flour.id,
            ingredient_name: "Flour".to_string(),
            unit: Some(Unit::Tablespoon),
            amount: Some(Decimal::from(3)),
        },
        RecipeIngredient {
            ingredient_id: yiest.id,
            ingredient_name: "Yiest".to_string(),
            unit: None,
            amount: Some(Decimal::from(1)),
        },
        RecipeIngredient {
            ingredient_id: water.id,
            ingredient_name: "Water".to_string(),
            unit: Some(Unit::Deciliter),
            amount: Some(Decimal::from(6)),
        },
    ];

    let recipe = CreateRecipe {
        user_id: created_user.id,
        name: "Pizza".to_string(),
        description: None,
        img: None,
        instructions: None,
        ingredients: ingredients.clone(),
        baking_time: NaiveTime::from_hms_opt(0, 20, 0),
        prep_time: NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
        servings: 4,
    };

    let recipe_id = db.create_recipe(&recipe).await?;
    let found_recipe = db.get_recipe(recipe_id).await?;

    assert_eq!(found_recipe.name, recipe.name);
    assert_eq!(ingredients, found_recipe.ingredients);

    Ok(())
}
