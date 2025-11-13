use super::utils::merge_class;
use dioxus::prelude::*;
#[component]
pub fn Progress(
    #[props(default = 0.0f32)] value: f32,
    #[props(default = 100.0f32)] max: f32,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let classes = merge_class("ui-progress", class);
    let percent = if max <= 0.0f32 {
        0.0
    } else {
        (value / max).clamp(0.0, 1.0) * 100.0
    };
    let indicator_style = format!("width: {percent:.2}%;");

    rsx! {
        div {
            class: classes,
            role: "progressbar",
            "aria-valuemin": 0,
            "aria-valuemax": max,
            "aria-valuenow": value,
            span {
                style: indicator_style,
            }
        }
    }
}
