use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct CreateRecipe {
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub img: String,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
#[cfg_attr(
    feature = "backend",
    derive(sqlx::Type),
    sqlx(type_name = "unit", rename_all = "lowercase")
)]
pub enum Unit {
    Milligram,
    Gram,
    Hectogram,
    Kilogram,
    Milliliter,
    Deciliter,
    Liter,
    Teaspoon,
    Tablespoon,
    Cup,
    Clove,
    Pinch,
}

#[cfg(feature = "backend")]
impl sqlx::postgres::PgHasArrayType for Unit {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_unit")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipeIngredient {
    pub ingredient_id: Uuid,
    pub unit: Option<Unit>,
    pub amount: Option<i32>,
}
