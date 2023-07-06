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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "unit"))]
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipeIngredient {
    pub ingredient_id: Uuid,
    pub unit: Option<Unit>,
    pub amount: Option<i32>,
}
