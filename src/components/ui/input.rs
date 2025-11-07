use dioxus::prelude::*;
use super::utils::merge_class;
#[component]
pub fn Input(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] value: Option<String>,
    #[props(into, default)] default_value: Option<String>,
    #[props(into, default)] placeholder: Option<String>,
    #[props(into, default)] name: Option<String>,
    #[props(into, default)] id: Option<String>,
    #[props(into, default)] autocomplete: Option<String>,
    #[props(into, default)] r#type: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] readonly: bool,
    #[props(default)] required: bool,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
    #[props(optional)] on_change: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = merge_class("ui-input", class);
    let input_handler = on_input.clone();
    let change_handler = on_change.clone();

    // Clone optional attributes so they can be moved into rsx
    let resolved_value = value.or(default_value).unwrap_or_default();
    let placeholder_attr = placeholder.unwrap_or_default();
    let name_attr = name.unwrap_or_default();
    let id_attr = id.unwrap_or_default();
    let autocomplete_attr = autocomplete.unwrap_or_default();

    rsx! {
        input {
            class: classes,
            r#type: r#type.unwrap_or_else(|| "text".to_string()),
            disabled,
            readonly,
            required,
            id: id_attr,
            name: name_attr,
            value: resolved_value,
            placeholder: placeholder_attr,
            autocomplete: autocomplete_attr,
            oninput: move |event| {
                if let Some(handler) = input_handler.clone() {
                    handler.call(event);
                }
            },
            onchange: move |event| {
                if let Some(handler) = change_handler.clone() {
                    handler.call(event);
                }
            },
        }
    }
}
