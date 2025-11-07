use dioxus::prelude::*;
use super::utils::merge_class;
#[component]
pub fn Skeleton(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] width: Option<String>,
    #[props(into, default)] height: Option<String>,
    #[props(into, default)] radius: Option<String>,
) -> Element {
    let classes = merge_class("ui-skeleton", class);

    let mut styles = Vec::new();
    if let Some(width) = width.filter(|value| !value.trim().is_empty()) {
        styles.push(format!("width: {width};"));
    }
    if let Some(height) = height.filter(|value| !value.trim().is_empty()) {
        styles.push(format!("height: {height};"));
    }
    if let Some(radius) = radius.filter(|value| !value.trim().is_empty()) {
        styles.push(format!("border-radius: {radius};"));
    }
    let style_attr = styles.join(" ");

    rsx! {
        div {
            class: classes,
            style: style_attr,
        }
    }
}
