use dioxus::prelude::*;
use super::utils::merge_class;
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

    #[allow(dead_code)]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[allow(dead_code)]
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

    rsx! {
        div {
            class: classes,
            role: "group",
            "aria-disabled": disabled,
            if let Some(label_text) = label.clone() {
                span {
                    class: "ui-choice-group-label",
                    "{label_text}"
                }
            }
            div {
                class: "ui-choice-group-options",
                for option in options {
                    {
                        let is_selected = current_values.iter().any(|item| item == &option.value);
                        let disabled_state = disabled || option.disabled;
                        rsx! {
                            CheckboxChipItem {
                                option,
                                is_selected,
                                disabled: disabled_state,
                                on_toggle: move |value: String| {
                                    values.with_mut(|items| {
                                        if let Some(index) = items.iter().position(|item| item == &value) {
                                            items.remove(index);
                                        } else {
                                            items.push(value);
                                        }
                                    });
                                    if let Some(callback) = on_values_change.as_ref() {
                                        callback.call(values());
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

#[component]
fn CheckboxChipItem(
    option: CheckboxChipOption,
    is_selected: bool,
    disabled: bool,
    on_toggle: EventHandler<String>,
) -> Element {
    let value = option.value.clone();

    rsx! {
        button {
            class: "ui-checkbox-chip",
            "data-state": if is_selected { "selected" } else { "idle" },
            "data-disabled": disabled,
            role: "checkbox",
            "aria-checked": if is_selected { "true" } else { "false" },
            "aria-disabled": if disabled { "true" } else { "false" },
            r#type: "button",
            disabled,
            onclick: move |_| {
                if !disabled {
                    on_toggle.call(value.clone());
                }
            },
            span {
                class: "ui-chip-label",
                "{option.label}"
            }
            if let Some(description) = option.description {
                span {
                    class: "ui-chip-description",
                    "{description}"
                }
            }
        }
    }
}
