use dioxus::html::events::FormEvent;
use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizableOrientation {
    Horizontal,
    Vertical,
}

impl ResizableOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            ResizableOrientation::Horizontal => "horizontal",
            ResizableOrientation::Vertical => "vertical",
        }
    }
}

impl Default for ResizableOrientation {
    fn default() -> Self {
        ResizableOrientation::Horizontal
    }
}

#[component]
pub fn ResizablePanels(
    first: Element,
    second: Element,
    #[props(default = 0.5f32)] initial: f32,
    #[props(default = 0.2f32)] min: f32,
    #[props(default = 0.8f32)] max: f32,
    #[props(default)] orientation: ResizableOrientation,
    #[props(optional)] on_resize: Option<EventHandler<f32>>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let min_clamped = min.clamp(0.05, 0.95);
    let max_clamped = max.clamp(min_clamped + f32::EPSILON, 0.95);
    let initial_ratio = initial.clamp(min_clamped, max_clamped);
    let ratio = use_signal(move || initial_ratio);
    let classes = merge_class("ui-resizable-panels", class);
    let orientation_attr = orientation.as_str();
    let on_resize_handler = on_resize.clone();
    let slider_min = (min_clamped * 100.0).round();
    let slider_max = (max_clamped * 100.0).round();

    let ratio_value = ratio();
    let first_basis = format!("{:.2}%", ratio_value * 100.0);
    let second_basis = format!("{:.2}%", (1.0 - ratio_value) * 100.0);
    let slider_value = format!("{:.0}", ratio_value * 100.0);

    rsx! {
        div {
            class: classes,
            "data-orientation": orientation_attr,
            div {
                class: "ui-resizable-pane",
                style: format!("flex-basis: {first_basis};"),
                {first}
            }
            div {
                class: "ui-resizable-handle-stack",
                div { class: "ui-resizable-handle" }
                input {
                    class: "ui-resizable-slider",
                    r#type: "range",
                    min: format!("{slider_min:.0}"),
                    max: format!("{slider_max:.0}"),
                    step: "1",
                    value: slider_value,
                    "aria-label": "Resize panels",
                    oninput: {
                        let mut ratio_signal = ratio.clone();
                        let min = min_clamped;
                        let max = max_clamped;
                        let handler = on_resize_handler.clone();
                        move |event: FormEvent| {
                            if let Ok(raw) = event.value().parse::<f32>() {
                                let value = (raw / 100.0).clamp(min, max);
                                ratio_signal.set(value);
                                if let Some(callback) = handler.clone() {
                                    callback.call(value);
                                }
                            }
                        }
                    },
                }
            }
            div {
                class: "ui-resizable-pane",
                style: format!("flex-basis: {second_basis};"),
                {second}
            }
        }
    }
}
