use chrono::{DateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct CreateRecipe {
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub img: Option<String>,
    pub servings: i32,
    pub prep_time: NaiveTime,
    pub baking_time: Option<NaiveTime>,
    pub ingredients: Vec<CreateRecipeIngredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub img: Option<String>,
    pub servings: i32,
    pub updated_at: DateTime<Utc>,
    // TODO: Hmm, I am not sure if I can use NaiveTime here if it's more than 24 hours prep time
    pub prep_time: NaiveTime,
    pub baking_time: Option<NaiveTime>,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateRecipeIngredient {
    pub ingredient_id: Uuid,
    pub unit: Option<Unit>,
    pub amount: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub ingredient_id: Uuid,
    pub ingredient_name: String,
    pub unit: Option<Unit>,
    pub amount: Option<Decimal>,
}
