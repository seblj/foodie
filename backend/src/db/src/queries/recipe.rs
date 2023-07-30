use common::{
    ingredient::{CreateIngredient, Ingredient},
    recipe::{CreateRecipe, Recipe, RecipeIngredient, Unit},
};
use sqlx::types::{Decimal, Uuid};

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_recipe(&self, create_recipe: &CreateRecipe) -> Result<Uuid, anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        let create_ingredients: Vec<CreateIngredient> =
            create_recipe.ingredients.iter().map(|i| i.into()).collect();

        self.create_ingredients(create_ingredients.clone()).await?;

        let names: Vec<String> = create_ingredients.into_iter().map(|i| i.name).collect();
        let ingredients =
            sqlx::query!("SELECT * FROM ingredients WHERE name = ANY($1)", &names[..])
                .map(|row| Ingredient {
                    id: row.id,
                    name: row.name,
                })
                .fetch_all(&mut *tx)
                .await?;

        // Ugly, but it gurantees correctnes in comparison to zip which _may_ use wrong ordering
        // after insert
        let ingredients: Vec<RecipeIngredient> = ingredients
            .into_iter()
            .filter_map(|i| {
                let ci = create_recipe
                    .ingredients
                    .iter()
                    .find(|ci| ci.ingredient_name == i.name)?;

                Some(RecipeIngredient {
                    ingredient_name: i.name,
                    unit: ci.unit,
                    amount: ci.amount,
                    ingredient_id: i.id,
                })
            })
            .collect();

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
        .fetch_one(&mut *tx)
        .await?;

        let (ids, units, amounts): (Vec<Uuid>, Vec<Option<Unit>>, Vec<Option<Decimal>>) =
            itertools::multiunzip(
                ingredients
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
            img: recipe.img,
            ingredients,
        })
    }
}
