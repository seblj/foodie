use common::ingredient::{CreateIngredient, Ingredient};

use crate::db::FoodiePool;

impl FoodiePool {
    pub async fn create_ingredient(
        &self,
        ingredient: &CreateIngredient,
    ) -> Result<Ingredient, anyhow::Error> {
        let created_ingredient = sqlx::query!(
            r#"
INSERT INTO
  ingredients (name, user_id)
VALUES
  ($1, $2)
ON CONFLICT (name) DO NOTHING
RETURNING
  id,
  name
    "#,
            ingredient.name,
            self.user_id,
        )
        .fetch_one(self)
        .await
        .unwrap();

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
        .fetch_all(self)
        .await?;

        Ok(created_ingredients)
    }
}
