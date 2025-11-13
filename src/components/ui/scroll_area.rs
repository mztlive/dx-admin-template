use super::utils::merge_class;
use dioxus::prelude::*;
#[component]
pub fn ScrollArea(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] style: Option<String>,
    #[props(into, default)] max_height: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-scroll-area", class);

    let mut styles = Vec::new();
    if let Some(style) = style.filter(|style| !style.trim().is_empty()) {
        styles.push(style);
    }
    if let Some(max_height) = max_height.filter(|height| !height.trim().is_empty()) {
        styles.push(format!("max-height: {max_height};"));
    }
    let style_attr = styles.join(" ");

    rsx! {
        div {
            class: classes,
            style: style_attr,
            {children}
        }
    }
}
