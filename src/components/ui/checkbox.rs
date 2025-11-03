use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[component]
pub fn Checkbox(
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(into, default)] id: Option<String>,
    #[props(into, default)] name: Option<String>,
    #[props(into, default)] value: Option<String>,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_checked_change: Option<EventHandler<bool>>,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
    #[props(optional)] on_change: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = merge_class("ui-checkbox", class);
    let checked_handler = on_checked_change.clone();
    let input_handler = on_input.clone();
    let change_handler = on_change.clone();
    let id_attr = id.unwrap_or_default();
    let name_attr = name.unwrap_or_default();
    let value_attr = value.unwrap_or_else(|| "on".to_string());

    rsx! {
        input {
            class: classes,
            r#type: "checkbox",
            role: "checkbox",
            checked,
            disabled,
            required,
            id: id_attr,
            name: name_attr,
            value: value_attr,
            oninput: move |event| {
                if let Some(handler) = checked_handler.clone() {
                    handler.call(event.checked());
                }
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
