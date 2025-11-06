use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[component]
pub fn Checkbox(
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(into, default)] id: Option<String>,
    #[props(into, default)] name: Option<String>,
    #[props(into, default)] value: Option<String>,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_checked_change: Option<EventHandler<bool>>,
    #[props(optional)] on_input: Option<EventHandler<FormEvent>>,
    #[props(optional)] on_change: Option<EventHandler<FormEvent>>,
) -> Element {
    let classes = merge_class("ui-checkbox", class);
    let checked_handler = on_checked_change.clone();
    let input_handler = on_input.clone();
    let change_handler = on_change.clone();
    let id_attr = id.unwrap_or_default();
    let name_attr = name.unwrap_or_default();
    let value_attr = value.unwrap_or_else(|| "on".to_string());

    rsx! {
        input {
            class: classes,
            r#type: "checkbox",
            role: "checkbox",
            checked,
            disabled,
            required,
            id: id_attr,
            name: name_attr,
            value: value_attr,
            oninput: move |event| {
                if let Some(handler) = checked_handler.clone() {
                    handler.call(event.checked());
                }
                if let Some(handler) = input_handler.clone() {
                    handler.call(event);
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

#[derive(Clone, PartialEq)]
pub struct CheckboxChipOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl CheckboxChipOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[component]
pub fn CheckboxChipGroup(
    mut values: Signal<Vec<String>>,
    #[props(into)] options: Vec<CheckboxChipOption>,
    #[props(into, default)] label: Option<String>,
    #[props(default)] disabled: bool,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_values_change: Option<EventHandler<Vec<String>>>,
) -> Element {
    let classes = merge_class("ui-checkbox-chip-group", class);
    let current_values = values();
    let group_disabled = disabled;

    rsx! {
        div {
            class: classes,
            role: "group",
            "aria-disabled": group_disabled,
            if let Some(label_text) = label.clone() {
                span {
                    class: "ui-choice-group-label",
                    "{label_text}"
                }
            }
            div {
                class: "ui-choice-group-options",
                for option in options.iter().cloned() {
                    {
                        let option_label = option.label.clone();
                        let option_value = option.value.clone();
                        let option_description = option.description.clone();
                        let is_disabled = group_disabled || option.disabled;
                        let is_selected = current_values.iter().any(|item| item == &option_value);
                        let mut values_signal = values.clone();
                        let handler = on_values_change.clone();

                        rsx! {
                            button {
                                class: "ui-checkbox-chip",
                                "data-state": if is_selected { "selected" } else { "idle" },
                                "data-disabled": is_disabled,
                                role: "checkbox",
                                "aria-checked": if is_selected { "true" } else { "false" },
                                "aria-disabled": if is_disabled { "true" } else { "false" },
                                r#type: "button",
                                disabled: is_disabled,
                                onclick: move |_| {
                                    if is_disabled {
                                        return;
                                    }
                                    values_signal.with_mut(|items| {
                                        if let Some(index) = items.iter().position(|item| item == &option_value) {
                                            items.remove(index);
                                        } else {
                                            items.push(option_value.clone());
                                        }
                                    });
                                    if let Some(callback) = handler.clone() {
                                        callback.call(values_signal());
                                    }
                                },
                                span {
                                    class: "ui-chip-label",
                                    "{option_label}"
                                }
                                if let Some(description) = option_description.clone() {
                                    span {
                                        class: "ui-chip-description",
                                        "{description}"
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
