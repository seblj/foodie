// use anyhow::anyhow;
// use common::recipe::{CreateRecipe, Unit};
// use sqlx::{postgres::PgQueryResult, types::Uuid};

// use crate::FoodieDatabase;

// impl FoodieDatabase {
// pub async fn create_recipe(
//     &self,
//     create_recipe: &CreateRecipe,
// ) -> Result<PgQueryResult, anyhow::Error> {
//     let mut tx = self.pool.begin().await?;

//     let recipe = sqlx::query!(
//         r#"
// INSERT INTO
// recipes (user_id, name, description, instructions, img)
// VALUES
// ($1, $2, $3, $4, $5)
// RETURNING
// id
// "#,
//         create_recipe.user_id,
//         create_recipe.name,
//         create_recipe.description,
//         create_recipe.instructions,
//         create_recipe.img
//     )
//     .fetch_one(&mut tx)
//     .await
//     .map_err(|_| anyhow!("Couldn't create an ingredient"))?;

//     sqlx::query!(
//         r#"
// INSERT INTO
// recipe_ingredients (recipe_id, ingredient_id, unit, amount)
// SELECT
// $1,
// *
// FROM
// UNNEST($2::UUID[], $3::unit[], $4::INTEGER[])
//     "#,
//         recipe.id,
//         create_recipe
//             .ingredients
//             .iter()
//             .map(|ingredient| ingredient.ingredient_id) as &[Uuid],
//         create_recipe
//             .ingredients
//             .iter()
//             .map(|ingredient| ingredient.unit) as &[Unit],
//         create_recipe
//             .ingredients
//             .iter()
//             .map(|ingredient| ingredient.amount) as &[i32],
//     )
//     .execute(&mut tx)
//     .await
//     .map_err(|_| anyhow!("Couldn't insert recipe ingredient"))
// }
// }
