use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::recipe::RecipeIngredient;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct CreateIngredient {
    pub name: String,
}

impl From<RecipeIngredient> for CreateIngredient {
    fn from(value: RecipeIngredient) -> Self {
        Self {
            name: value.ingredient_name,
        }
    }
}

impl From<&RecipeIngredient> for CreateIngredient {
    fn from(value: &RecipeIngredient) -> Self {
        Self {
            name: value.ingredient_name.clone(),
        }
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
}
