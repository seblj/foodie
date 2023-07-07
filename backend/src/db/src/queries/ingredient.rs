use anyhow::anyhow;
use common::ingredient::{CreateIngredient, Ingredient};

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_ingredient(
        &self,
        create_ingredient: &CreateIngredient,
    ) -> Result<Ingredient, anyhow::Error> {
        let ingredient = sqlx::query!(
            r#"
INSERT INTO
  ingredients (name)
VALUES
  ($1)
RETURNING
  id,
  name
    "#,
            create_ingredient.name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| anyhow!("Couldn't create an ingredient"))?;

        Ok(Ingredient {
            id: ingredient.id,
            name: ingredient.name,
        })
    }
}
