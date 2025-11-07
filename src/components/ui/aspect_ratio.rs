use dioxus::prelude::*;
use super::utils::merge_class;
#[component]
pub fn AspectRatio(
    #[props(default = 1.0f32)] ratio: f32,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let clamped_ratio = if ratio <= 0.0 { 1.0 } else { ratio };
    let classes = merge_class("ui-aspect-ratio", class);

    rsx! {
        div {
            class: classes,
            style: format!("--ui-aspect-ratio: {clamped_ratio};"),
            div {
                class: "ui-aspect-ratio-inner",
                {children}
            }
        }
    }
}
