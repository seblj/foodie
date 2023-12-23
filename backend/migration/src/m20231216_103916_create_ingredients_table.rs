use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingredients::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ingredients::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Ingredients::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Ingredients::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ingredients-user_id")
                            .from(Ingredients::Table, Ingredients::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ingredients::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Ingredients {
    Table,
    Id,
    Name,
    UserId,
}
