use sqlx::PgPool;

pub mod models;
mod queries;

pub use queries::*;

#[derive(Clone)]
pub struct FoodieDatabase {
    pub(crate) pool: PgPool,
}

impl FoodieDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
