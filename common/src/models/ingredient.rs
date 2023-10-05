use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct CreateIngredient {
    pub name: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
}
