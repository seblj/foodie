use anyhow::anyhow;
use common::recipe::{CreateRecipe, Recipe, RecipeIngredient, Unit};
use sqlx::types::Uuid;

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_recipe(&self, create_recipe: &CreateRecipe) -> Result<Uuid, anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        let recipe = sqlx::query!(
            r#"
INSERT INTO
  recipes (user_id, name, description, instructions, img)
VALUES
  ($1, $2, $3, $4, $5)
RETURNING
  id
"#,
            create_recipe.user_id,
            create_recipe.name,
            create_recipe.description,
            create_recipe.instructions,
            create_recipe.img
        )
        .fetch_one(&mut tx)
        .await
        .map_err(|_| anyhow!("Couldn't create an ingredient"))?;

        let ids = &create_recipe
            .ingredients
            .iter()
            .map(|ingredient| ingredient.ingredient_id)
            .collect::<Vec<Uuid>>();

        let units = &create_recipe
            .ingredients
            .iter()
            .map(|ingredient| ingredient.unit)
            .collect::<Vec<Option<Unit>>>();

        let amounts = &create_recipe
            .ingredients
            .iter()
            .map(|ingredient| ingredient.amount)
            .collect::<Vec<Option<i32>>>();

        sqlx::query!(
            r#"
INSERT INTO
  recipe_ingredients (recipe_id, ingredient_id, unit, amount)
SELECT
  $1,
  *
FROM
  UNNEST($2::UUID[], $3::unit[], $4::INTEGER[])
    "#,
            recipe.id,
            ids,
            units as &[Option<Unit>],
            amounts as &[Option<i32>],
        )
        .execute(&mut tx)
        .await?;

        tx.commit()
            .await
            .map_err(|_| anyhow!("Couldn't commit recipe"))?;

        Ok(recipe.id)
    }

    pub async fn delete_recipe(&self, recipe_id: Uuid) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
DELETE FROM recipes
WHERE
  id = $1
        "#,
            recipe_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recipe(&self, recipe_id: Uuid) -> Result<Recipe, anyhow::Error> {
        let recipe = sqlx::query!(
            r#"
SELECT
  *
FROM
  recipes
WHERE
  id = $1
        "#,
            recipe_id
        )
        .fetch_one(&self.pool)
        .await?;

        let ingredients = sqlx::query!(
            r#"
SELECT
  ri.unit AS "unit: Unit",
  ri.amount,
  i.name,
  i.id
FROM
  recipe_ingredients ri
  JOIN ingredients i ON ri.ingredient_id = i.id
WHERE
  recipe_id = $1
        "#,
            recipe_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|ingredient| RecipeIngredient {
            ingredient_id: ingredient.id,
            ingredient_name: ingredient.name,
            unit: ingredient.unit,
            amount: ingredient.amount,
        })
        .collect();

        Ok(Recipe {
            id: recipe.id,
            user_id: recipe.user_id,
            name: recipe.name,
            description: recipe.description,
            instructions: recipe.instructions,
            img: recipe.img,
            ingredients,
        })
    }
}
