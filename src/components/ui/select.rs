use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

#[component]
pub fn Select(
    #[props(into, default)] id: Option<String>,
    #[props(into)] placeholder: String,
    #[props(into)] options: Vec<SelectOption>,
    #[props(into, default)] selected: Option<String>,
    #[props(default)] disabled: bool,
    #[props(optional)] on_change: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| false);
    let current = use_signal(move || selected.clone());
    let on_change_handler = on_change.clone();
    let trigger_id = id.unwrap_or_default();

    let selected_value = current();
    let display_text = selected_value
        .as_ref()
        .and_then(|value| {
            options
                .iter()
                .find(|option| option.value == *value)
                .map(|option| option.label.clone())
        })
        .unwrap_or_else(|| placeholder.clone());

    rsx! {
        div {
            class: "ui-select",
            "data-disabled": disabled,
            onclick: move |event| {
                event.stop_propagation();
            },
            button {
                class: "ui-select-trigger",
                "data-open": if open() { "true" } else { "false" },
                disabled,
                id: trigger_id.clone(),
                "aria-haspopup": "listbox",
                "aria-expanded": if open() { "true" } else { "false" },
                onclick: {
                    let mut open_signal = open.clone();
                    move |_| {
                        if !disabled {
                            let new_state = !open_signal();
                            open_signal.set(new_state);
                        }
                    }
                },
                span { "{display_text}" }
                span {
                    class: "ui-select-icon",
                    "aria-hidden": "true",
                    svg {
                        view_box: "0 0 16 16",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M4 6l4 4 4-4",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "1.5",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                        }
                    }
                }
            }
            if open() {
                div {
                    class: "ui-select-content",
                    onclick: move |event| {
                        event.stop_propagation();
                    },
                    div {
                        class: "ui-select-list",
                        for option in options.iter().cloned() {
                            {
                                let is_active = selected_value
                                    .as_ref()
                                    .map(|value| value == &option.value)
                                    .unwrap_or(false);
                                let value = option.value.clone();
                                let handler = on_change_handler.clone();
                                let mut open_signal = open.clone();
                                let mut current_signal = current.clone();

                                rsx! {
                                    button {
                                        class: "ui-select-item",
                                        "data-state": if is_active { "active" } else { "inactive" },
                                        onclick: {
                                            let value = value.clone();
                                            let handler = handler.clone();
                                            move |_| {
                                                current_signal.set(Some(value.clone()));
                                                if let Some(callback) = handler.clone() {
                                                    callback.call(value.clone());
                                                }
                                                open_signal.set(false);
                                            }
                                        },
                                        span { "{option.label}" }
                                        if is_active {
                                            span {
                                                style: "font-size: 0.75rem; opacity: 0.7;",
                                                "✓"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
