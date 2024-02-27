mod models;

use std::fmt::Display;

use form_derive::FormFieldValues;
use serde::{Deserialize, Serialize};

pub use models::*;

// Re-export some strum types I use in frontend crate.
// Export them under the mod `strum` to not get confused by `use common::Display`.
// So instead it is `use common::strum::Display`
pub mod strum {
    pub use strum::IntoEnumIterator;
}

macro_rules! form_field_value_impl {
    ($name:ident, $ty:ty) => {
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub struct $name;
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($name))
            }
        }

        impl FormFieldValues<$ty> for $name {}
    };
}

form_field_value_impl!(FormFieldValueString, String);
form_field_value_impl!(FormFieldValueBool, bool);
