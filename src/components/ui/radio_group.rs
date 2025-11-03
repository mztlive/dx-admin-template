use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

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
