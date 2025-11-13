use std::sync::atomic::{AtomicUsize, Ordering};

use super::utils::merge_class;
use dioxus::prelude::*;
static RADIO_GROUP_IDS: AtomicUsize = AtomicUsize::new(0);

fn next_radio_group_name() -> String {
    let id = RADIO_GROUP_IDS.fetch_add(1, Ordering::Relaxed);
    format!("radio-group-{id}")
}

#[derive(Clone)]
struct RadioGroupContext {
    name: Signal<String>,
    value: Signal<Option<String>>,
    disabled: bool,
    on_change: Option<EventHandler<String>>,
}

#[component]
pub fn RadioGroup(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] name: Option<String>,
    #[props(into, default)] default_value: Option<String>,
    #[props(default)] disabled: bool,
    #[props(optional)] on_value_change: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let provided_name = name.clone();
    let group_name = use_signal(move || {
        provided_name
            .clone()
            .unwrap_or_else(|| next_radio_group_name())
    });
    let initial_value = default_value.clone();
    let selected = use_signal(move || initial_value.clone());

    let context = RadioGroupContext {
        name: group_name.clone(),
        value: selected.clone(),
        disabled,
        on_change: on_value_change.clone(),
    };

    use_context_provider(|| context);

    let classes = merge_class("ui-radio-group", class);

    rsx! {
        div {
            class: classes,
            role: "radiogroup",
            "aria-disabled": disabled,
            {children}
        }
    }
}

#[component]
pub fn RadioGroupItem(
    #[props(into)] value: String,
    #[props(default)] disabled: bool,
    #[props(into, default)] id: Option<String>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let context = use_context::<RadioGroupContext>();
    let classes = merge_class("ui-radio", class);
    let id_attr = id.unwrap_or_default();
    let is_disabled = disabled || context.disabled;
    let mut group_value_signal = context.value.clone();
    let current_value = group_value_signal();
    let is_selected = current_value.as_ref() == Some(&value);
    let group_name_signal = context.name.clone();
    let group_name = group_name_signal();
    let value_attr = value.clone();
    let value_for_handler = value.clone();
    let on_change = context.on_change.clone();

    rsx! {
        input {
            class: classes,
            r#type: "radio",
            role: "radio",
            name: "{group_name}",
            value: "{value_attr}",
            checked: is_selected,
            disabled: is_disabled,
            id: format_args!("{}", id_attr),
            onchange: move |_| {
                group_value_signal.set(Some(value_for_handler.clone()));
                if let Some(handler) = on_change.clone() {
                    handler.call(value_for_handler.clone());
                }
            },
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct RadioChipOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl RadioChipOption {
    #[allow(dead_code)]
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
pub fn RadioChipGroup(
    mut value: Signal<Option<String>>,
    #[props(into)] options: Vec<RadioChipOption>,
    #[props(into, default)] label: Option<String>,
    #[props(default)] disabled: bool,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_value_change: Option<EventHandler<String>>,
) -> Element {
    let classes = merge_class("ui-radio-chip-group", class);
    let current_value = value();

    rsx! {
        div {
            class: classes,
            role: "radiogroup",
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
                        let is_selected = current_value.as_ref().map(|v| v == &option.value).unwrap_or(false);
                        let disabled_state = disabled || option.disabled;
                        rsx! {
                            RadioChipItem {
                                option,
                                is_selected,
                                disabled: disabled_state,
                                on_select: move |selected_value: String| {
                                    value.set(Some(selected_value.clone()));
                                    if let Some(callback) = on_value_change.as_ref() {
                                        callback.call(selected_value);
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
fn RadioChipItem(
    option: RadioChipOption,
    is_selected: bool,
    disabled: bool,
    on_select: EventHandler<String>,
) -> Element {
    let value = option.value.clone();

    rsx! {
        button {
            class: "ui-radio-chip",
            "data-state": if is_selected { "selected" } else { "idle" },
            "data-disabled": disabled,
            role: "radio",
            "aria-checked": if is_selected { "true" } else { "false" },
            "aria-disabled": if disabled { "true" } else { "false" },
            r#type: "button",
            disabled,
            onclick: move |_| {
                if !disabled && !is_selected {
                    on_select.call(value.clone());
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
