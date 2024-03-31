pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20231216_004843_create_recipes_table;
mod m20231216_103342_create_unit_type;
mod m20231216_103916_create_ingredients_table;
mod m20231216_104607_create_recipe_ingredients_table;
mod m20240218_134359_instructions_as_vec;
mod m20240331_125419_image_to_uuid_column;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20231216_004843_create_recipes_table::Migration),
            Box::new(m20231216_103342_create_unit_type::Migration),
            Box::new(m20231216_103916_create_ingredients_table::Migration),
            Box::new(m20231216_104607_create_recipe_ingredients_table::Migration),
            Box::new(m20240218_134359_instructions_as_vec::Migration),
            Box::new(m20240331_125419_image_to_uuid_column::Migration),
        ]
    }
}
