use common::recipe::{CreateRecipe, Recipe, RecipeIngredient, Unit};
use sqlx::types::{Decimal, Uuid};

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_recipe(&self, create_recipe: &CreateRecipe) -> Result<Uuid, anyhow::Error> {
        let mut tx = self.pool.begin().await?;
        let recipe = sqlx::query!(
            r#"
INSERT INTO
  recipes (
    user_id,
    name,
    description,
    instructions,
    img,
    servings,
    prep_time,
    baking_time
  )
VALUES
  ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING
  id
"#,
            create_recipe.user_id,
            create_recipe.name,
            create_recipe.description,
            create_recipe.instructions,
            create_recipe.img,
            create_recipe.servings,
            create_recipe.prep_time,
            create_recipe.baking_time
        )
        .fetch_one(&mut *tx)
        .await?;

        let (ids, units, amounts): (Vec<Uuid>, Vec<Option<Unit>>, Vec<Option<Decimal>>) =
            itertools::multiunzip(
                create_recipe
                    .ingredients
                    .iter()
                    .map(|r| (r.ingredient_id, r.unit, r.amount)),
            );

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
            &ids,
            &units as &[Option<Unit>],
            &amounts as &[Option<Decimal>],
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

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

    pub async fn get_recipe_ingredients(
        &self,
        recipe_id: Uuid,
    ) -> Result<Vec<RecipeIngredient>, anyhow::Error> {
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

        Ok(ingredients)
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

        let ingredients = self.get_recipe_ingredients(recipe_id).await?;

        Ok(Recipe {
            id: recipe.id,
            user_id: recipe.user_id,
            name: recipe.name,
            description: recipe.description,
            instructions: recipe.instructions,
            updated_at: recipe.updated_at,
            prep_time: recipe.prep_time,
            img: recipe.img,
            ingredients,
            servings: recipe.servings,
            baking_time: recipe.baking_time,
        })
    }
}
