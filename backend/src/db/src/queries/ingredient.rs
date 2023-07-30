use anyhow::anyhow;
use common::ingredient::{CreateIngredient, Ingredient};

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_ingredient(
        &self,
        ingredient: &CreateIngredient,
    ) -> Result<Ingredient, anyhow::Error> {
        let created_ingredient = sqlx::query!(
            r#"
INSERT INTO
  ingredients (name)
VALUES
  ($1)
ON CONFLICT (name) DO NOTHING
RETURNING
  id,
  name
    "#,
            ingredient.name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| anyhow!("Couldn't create an ingredient"))?;

        Ok(Ingredient {
            id: created_ingredient.id,
            name: created_ingredient.name,
        })
    }

    pub async fn create_ingredients(
        &self,
        ingredients: Vec<CreateIngredient>,
    ) -> Result<Vec<Ingredient>, anyhow::Error> {
        let names = ingredients
            .into_iter()
            .map(|i| i.name)
            .collect::<Vec<String>>();

        let created_ingredients = sqlx::query!(
            r#"
INSERT INTO
  ingredients (name)
SELECT
  *
FROM
  UNNEST($1::TEXT[])
ON CONFLICT (name) DO NOTHING
RETURNING
  id,
  name
    "#,
            &names,
        )
        .map(|row| Ingredient {
            id: row.id,
            name: row.name,
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(created_ingredients)
    }
}
