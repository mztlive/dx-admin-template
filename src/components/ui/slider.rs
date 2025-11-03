use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[component]
pub fn Slider(
    #[props(default = 0.0f32)] value: f32,
    #[props(default = 0.0f32)] min: f32,
    #[props(default = 100.0f32)] max: f32,
    #[props(default = 1.0f32)] step: f32,
    #[props(default)] disabled: bool,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_value_change: Option<EventHandler<f32>>,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
    #[props(optional)] on_change: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = merge_class("ui-slider", class);
    let percent = if (max - min).abs() <= f32::EPSILON {
        0.0
    } else {
        ((value - min) / (max - min)).clamp(0.0, 1.0) * 100.0
    };
    let style = format!("--fill: {percent:.2}%;");
    let value_change = on_value_change.clone();
    let input_handler = on_input.clone();
    let change_handler = on_change.clone();

    rsx! {
        div {
            class: classes,
            input {
                r#type: "range",
                min: min,
                max: max,
                step: step,
                value: value,
                disabled,
                style: style,
                oninput: move |event| {
                    if let Some(handler) = input_handler.clone() {
                        handler.call(event.clone());
                    }
                    if let Some(handler) = value_change.clone() {
                        if let Ok(parsed) = event.value().parse::<f32>() {
                            handler.call(parsed);
                        }
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
}
