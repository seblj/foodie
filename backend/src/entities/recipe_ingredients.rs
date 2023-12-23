//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use super::sea_orm_active_enums::Unit;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "recipe_ingredients")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub recipe_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub ingredient_id: i32,
    pub unit: Option<Unit>,
    pub amount: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ingredients::Entity",
        from = "Column::IngredientId",
        to = "super::ingredients::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Ingredients,
    #[sea_orm(
        belongs_to = "super::recipes::Entity",
        from = "Column::RecipeId",
        to = "super::recipes::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Recipes,
}

impl Related<super::ingredients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingredients.def()
    }
}

impl Related<super::recipes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
