use common::{
    ingredient::CreateIngredient,
    recipe::{CreateRecipe, Recipe, RecipeIngredient, Unit},
    user::CreateUser,
};
use db::FoodieDatabase;
use sqlx::PgPool;

#[sqlx::test(migrations = "../db/migrations")]
async fn create_and_get_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
    let db = FoodieDatabase::new(pool);

    let user = CreateUser {
        name: "foo".to_string(),
        email: "foo@foo.com".to_string(),
    };

    let created_user = db.create_user(&user).await?;

    let flour = db
        .create_ingredient(&CreateIngredient {
            name: "Flour".to_string(),
        })
        .await?;

    let ingredients = vec![RecipeIngredient {
        ingredient_id: flour.id,
        ingredient_name: flour.name,
        unit: Some(Unit::Tablespoon),
        amount: Some(3),
    }];

    let recipe = CreateRecipe {
        user_id: created_user.id,
        name: "Pizza".to_string(),
        description: None,
        img: None,
        instructions: None,
        ingredients: ingredients.clone(),
    };

    let recipe_id = db.create_recipe(&recipe).await?;

    assert_eq!(
        db.get_recipe(recipe_id).await?,
        Recipe {
            id: recipe_id,
            name: recipe.name,
            user_id: created_user.id,
            img: None,
            description: None,
            instructions: None,
            ingredients,
        },
    );

    Ok(())
}
