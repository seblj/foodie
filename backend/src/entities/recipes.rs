//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "recipes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub img: Option<String>,
    pub servings: i32,
    pub prep_time: Option<Time>,
    pub baking_time: Option<Time>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::recipe_ingredients::Entity")]
    RecipeIngredients,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::recipe_ingredients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeIngredients.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::ingredients::Entity> for Entity {
    fn to() -> RelationDef {
        super::recipe_ingredients::Relation::Ingredients.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::recipe_ingredients::Relation::Recipes.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
