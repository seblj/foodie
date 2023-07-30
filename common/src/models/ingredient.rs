use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::recipe::CreateRecipeIngredient;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct CreateIngredient {
    pub name: String,
}

impl From<CreateRecipeIngredient> for CreateIngredient {
    fn from(value: CreateRecipeIngredient) -> Self {
        Self {
            name: value.ingredient_name,
        }
    }
}

impl From<&CreateRecipeIngredient> for CreateIngredient {
    fn from(value: &CreateRecipeIngredient) -> Self {
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
