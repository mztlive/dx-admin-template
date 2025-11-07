use dioxus::prelude::*;
use super::utils::merge_class;
#[component]
pub fn Label(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] html_for: Option<String>,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    let classes = merge_class("ui-label", class);

    let html_for_attr = html_for.unwrap_or_default();

    rsx! {
        label {
            class: classes,
            "data-disabled": disabled,
            r#for: html_for_attr,
            {children}
        }
    }
}
