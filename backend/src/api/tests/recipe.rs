use common::{
    ingredient::CreateIngredient,
    recipe::{CreateRecipe, RecipeIngredient, Unit},
    user::CreateUser,
};
use db::FoodieDatabase;
use sqlx::PgPool;

#[sqlx::test(migrations = "../db/migrations")]
async fn create_recipe(pool: PgPool) -> Result<(), anyhow::Error> {
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
            unit: None,
            amount: Some(3),
        },
        RecipeIngredient {
            ingredient_id: butter.id,
            unit: Some(Unit::Tablespoon),
            amount: Some(1),
        },
        RecipeIngredient {
            ingredient_id: salt.id,
            unit: None,
            amount: None,
        },
        RecipeIngredient {
            ingredient_id: pepper.id,
            unit: None,
            amount: None,
        },
    ];

    let recipe = CreateRecipe {
        user_id: created_user.id,
        name: "Omelet".to_string(),
        description: "Desc".to_string(),
        img: "".to_string(),
        instructions: "instructions".to_string(),
        ingredients,
    };

    db.create_recipe(&recipe).await?;

    Ok(())
}
