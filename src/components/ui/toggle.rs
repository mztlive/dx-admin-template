use super::utils::merge_class;
use dioxus::prelude::*;
#[component]
pub fn Toggle(
    #[props(default)] pressed: bool,
    #[props(default)] disabled: bool,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_pressed_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-toggle", class);
    let handler = on_pressed_change.clone();

    rsx! {
        button {
            class: classes,
            "data-state": if pressed { "on" } else { "off" },
            "aria-pressed": pressed,
            disabled,
            onclick: move |_| {
                if let Some(callback) = handler.clone() {
                    callback.call(!pressed);
                }
            },
            {children}
        }
    }
}
