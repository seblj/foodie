use anyhow::anyhow;
use common::recipe::{CreateRecipe, Unit};
use sqlx::types::Uuid;

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_recipe(&self, create_recipe: &CreateRecipe) -> Result<(), anyhow::Error> {
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
            .map_err(|_| anyhow!("Couldn't commit recipe"))
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
}
