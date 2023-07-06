use sqlx::PgPool;

mod models;
mod queries;

pub use models::*;

#[derive(Clone)]
pub struct FoodieDatabase {
    pub(crate) pool: PgPool,
}

impl FoodieDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
