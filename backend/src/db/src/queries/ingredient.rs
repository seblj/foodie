use anyhow::anyhow;
use common::ingredient::CreateIngredient;
use sqlx::postgres::PgQueryResult;

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_ingredient(
        &self,
        create_ingredient: &CreateIngredient,
    ) -> Result<PgQueryResult, anyhow::Error> {
        sqlx::query!(
            r#"
INSERT INTO
  ingredients (name)
VALUES
  ($1)
    "#,
            create_ingredient.name,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| anyhow!("Couldn't create an ingredient"))
    }
}
