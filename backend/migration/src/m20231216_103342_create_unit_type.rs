use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Unit::Table)
                    .values([
                        Unit::Milligram,
                        Unit::Gram,
                        Unit::Hectogram,
                        Unit::Kilogram,
                        Unit::Milliliter,
                        Unit::Deciliter,
                        Unit::Liter,
                        Unit::Teaspoon,
                        Unit::Tablespoon,
                        Unit::Cup,
                        Unit::Clove,
                        Unit::Pinch,
                    ])
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(Unit::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Unit {
    Table,
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
