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

    let egg = db
        .create_ingredient(&CreateIngredient {
            name: "Egg".to_string(),
        })
        .await?;
    let butter = db
        .create_ingredient(&CreateIngredient {
            name: "Butter".to_string(),
        })
        .await?;
    let salt = db
        .create_ingredient(&CreateIngredient {
            name: "Salt".to_string(),
        })
        .await?;
    let pepper = db
        .create_ingredient(&CreateIngredient {
            name: "Pepper".to_string(),
        })
        .await?;

    let ingredients = vec![
        RecipeIngredient {
            ingredient_id: egg.id,
            ingredient_name: egg.name,
            unit: None,
            amount: Some(3),
        },
        RecipeIngredient {
            ingredient_id: butter.id,
            ingredient_name: butter.name,
            unit: Some(Unit::Tablespoon),
            amount: Some(1),
        },
        RecipeIngredient {
            ingredient_id: salt.id,
            ingredient_name: salt.name,
            unit: None,
            amount: None,
        },
        RecipeIngredient {
            ingredient_id: pepper.id,
            ingredient_name: pepper.name,
            unit: None,
            amount: None,
        },
    ];

    let recipe = CreateRecipe {
        user_id: created_user.id,
        name: "Omelet".to_string(),
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
