use super::utils::merge_class;
use dioxus::prelude::*;
#[component]
pub fn Textarea(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] value: Option<String>,
    #[props(into, default)] placeholder: Option<String>,
    #[props(into, default)] name: Option<String>,
    #[props(into, default)] id: Option<String>,
    #[props(into, default)] rows: Option<u16>,
    #[props(default)] disabled: bool,
    #[props(default)] readonly: bool,
    #[props(default)] required: bool,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
    #[props(optional)] on_change: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = merge_class("ui-textarea", class);
    let input_handler = on_input.clone();
    let change_handler = on_change.clone();
    let resolved_value = value.unwrap_or_default();
    let placeholder_attr = placeholder.unwrap_or_default();
    let name_attr = name.unwrap_or_default();
    let id_attr = id.unwrap_or_default();
    let rows_attr = rows.unwrap_or(5);

    rsx! {
        textarea {
            class: classes,
            disabled,
            readonly,
            required,
            rows: rows_attr,
            id: id_attr,
            name: name_attr,
            value: resolved_value,
            placeholder: placeholder_attr,
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
