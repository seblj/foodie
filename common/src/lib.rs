mod models;

pub use models::*;

// Re-export some strum types I use in frontend crate.
// Export them under the mod `strum` to not get confused by `use common::Display`.
// So instead it is `use common::strum::Display`
pub mod strum {
    pub use strum::IntoEnumIterator;
}
