use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct CreateRecipe {
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub img: Option<String>,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub img: Option<String>,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RecipeIngredient {
    pub ingredient_id: Uuid,
    pub ingredient_name: String,
    pub unit: Option<Unit>,
    pub amount: Option<i32>,
}
