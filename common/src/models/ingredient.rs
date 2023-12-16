use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateIngredient {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}
