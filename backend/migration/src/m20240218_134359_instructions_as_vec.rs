use sea_orm_migration::prelude::*;

use crate::m20231216_004843_create_recipes_table::Recipes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Recipes::Table)
                    .add_column(ColumnDef::new(Alias::new("foo")).array(ColumnType::Text))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Recipes::Table)
                    .drop_column(Recipes::Instructions)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Recipes::Table)
                    .rename_column(Alias::new("foo"), Recipes::Instructions)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
