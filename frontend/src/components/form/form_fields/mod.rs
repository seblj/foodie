pub mod form_field_checkbox;
pub mod form_field_duration;
pub mod form_field_input;
pub mod form_field_list;
pub mod form_field_select;
pub mod form_field_textarea;

pub(super) fn get_span(default: &str) -> String {
    let mut default_span = default
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let col_span = default_span.iter().find(|s| s.starts_with("col-span"));
    if col_span.is_none() {
        default_span.push("col-span-12".to_string());
    }

    default_span.join(" ")
}
