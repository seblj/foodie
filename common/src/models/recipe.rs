use chrono::{DateTime, FixedOffset, NaiveTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreateRecipe {
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<Vec<String>>,
    pub img: Option<String>,
    pub servings: i32,
    pub prep_time: Option<NaiveTime>,
    pub baking_time: Option<NaiveTime>,
    pub ingredients: Vec<CreateRecipeIngredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<Vec<String>>,
    pub img: Option<String>,
    pub servings: i32,
    pub updated_at: DateTime<FixedOffset>,
    pub prep_time: Option<NaiveTime>,
    pub baking_time: Option<NaiveTime>,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Eq, PartialEq, EnumIter, Display)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct CreateRecipeIngredient {
    pub name: String,
    pub unit: Option<Unit>,
    pub amount: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecipeIngredient {
    pub ingredient_id: i32,
    pub ingredient_name: String,
    pub unit: Option<Unit>,
    pub amount: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecipeImage {
    pub id: String,
    pub url: String,
}

impl From<Recipe> for CreateRecipe {
    fn from(recipe: Recipe) -> Self {
        Self {
            name: recipe.name,
            description: recipe.description,
            instructions: recipe.instructions,
            img: recipe.img,
            servings: recipe.servings,
            prep_time: recipe.prep_time,
            baking_time: recipe.baking_time,
            ingredients: recipe
                .ingredients
                .into_iter()
                .map(|ri| CreateRecipeIngredient::from(ri))
                .collect(),
        }
    }
}

impl From<RecipeIngredient> for CreateRecipeIngredient {
    fn from(recipe_ingredient: RecipeIngredient) -> Self {
        Self {
            name: recipe_ingredient.ingredient_name,
            unit: recipe_ingredient.unit,
            amount: recipe_ingredient.amount,
        }
    }
}
