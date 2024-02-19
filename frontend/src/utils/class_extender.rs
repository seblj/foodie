use leptos::{AttributeValue, IntoAttribute};

pub trait ExtendClass {
    fn extend_class(self, s: &str) -> String;
}

impl ExtendClass for Option<AttributeValue> {
    fn extend_class(self, s: &str) -> String {
        let attr = self.into_attribute();
        if let Some(c) = attr.as_nameless_value_string() {
            format!("{c} {s}")
        } else {
            s.to_string()
        }
    }
}
