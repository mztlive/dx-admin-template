use super::utils::merge_class;
use dioxus::prelude::*;
#[component]
pub fn Switch(
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(into, default)] id: Option<String>,
    #[props(into, default)] name: Option<String>,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_checked_change: Option<EventHandler<bool>>,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
    #[props(optional)] on_change: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = merge_class("ui-switch", class);
    let checked_handler = on_checked_change.clone();
    let input_handler = on_input.clone();
    let change_handler = on_change.clone();
    let id_attr = id.unwrap_or_default();
    let name_attr = name.unwrap_or_default();

    rsx! {
        input {
            class: classes,
            r#type: "checkbox",
            role: "switch",
            checked,
            disabled,
            "aria-checked": checked,
            id: id_attr,
            name: name_attr,
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
