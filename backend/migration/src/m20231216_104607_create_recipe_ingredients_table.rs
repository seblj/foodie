use sea_orm_migration::prelude::*;

use crate::{
    m20231216_004843_create_recipes_table::Recipes, m20231216_103342_create_unit_type::Unit,
    m20231216_103916_create_ingredients_table::Ingredients,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecipeIngredients::Table)
                    .if_not_exists()
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_ingredients-recipe_id")
                            .from(RecipeIngredients::Table, RecipeIngredients::RecipeId)
                            .to(Recipes::Table, Recipes::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_ingredients-ingredient_id")
                            .from(RecipeIngredients::Table, RecipeIngredients::IngredientId)
                            .to(Ingredients::Table, Ingredients::Id),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredients::RecipeId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredients::IngredientId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        sea_query::Index::create()
                            .col(RecipeIngredients::RecipeId)
                            .col(RecipeIngredients::IngredientId),
                    )
                    .col(ColumnDef::new(RecipeIngredients::Unit).custom(Unit::Table))
                    .col(ColumnDef::new(RecipeIngredients::Amount).decimal())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecipeIngredients::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RecipeIngredients {
    Table,
    RecipeId,
    IngredientId,
    Unit,
    Amount,
}
