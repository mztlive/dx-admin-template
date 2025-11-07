use dioxus::prelude::*;
use super::utils::merge_class;

/// Horizontal or vertical separator/divider component.
/// 
/// # Example
/// ```rust
/// rsx! {
///     Separator { orientation: SeparatorOrientation::Horizontal }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "horizontal",
            SeparatorOrientation::Vertical => "vertical",
        }
    }
}

impl Default for SeparatorOrientation {
    fn default() -> Self {
        SeparatorOrientation::Horizontal
    }
}

#[component]
pub fn Separator(
    #[props(default)] orientation: SeparatorOrientation,
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] style: Option<String>,
) -> Element {
    let classes = merge_class("ui-separator", class);
    let style_attr = style.unwrap_or_default();

    rsx! {
        div {
            class: classes,
            role: "separator",
            "data-orientation": orientation.as_str(),
            "aria-orientation": orientation.as_str(),
            style: style_attr,
        }
    }
}
