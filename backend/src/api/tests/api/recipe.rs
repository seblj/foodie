use common::{
    recipe::{CreateRecipe, CreateRecipeIngredient, Unit},
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

    let ingredients = vec![
        CreateRecipeIngredient {
            ingredient_name: "Flour".to_string(),
            unit: Some(Unit::Tablespoon),
            amount: Some(Decimal::from(3)),
        },
        CreateRecipeIngredient {
            ingredient_name: "Yiest".to_string(),
            unit: None,
            amount: Some(Decimal::from(1)),
        },
        CreateRecipeIngredient {
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
    };

    let recipe_id = db.create_recipe(&recipe).await?;

    let found_recipe = db.get_recipe(recipe_id).await?;
    let found_ingredients: Vec<CreateRecipeIngredient> =
        found_recipe.ingredients.iter().map(|i| i.into()).collect();

    assert_eq!(found_recipe.name, recipe.name);
    assert_eq!(ingredients, found_ingredients);

    Ok(())
}
