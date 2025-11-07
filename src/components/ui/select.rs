use dioxus::prelude::*;
use std::rc::Rc;

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
    let mut current = use_signal(move || selected.clone());
    let mut container_ref = use_signal(|| None as Option<Rc<MountedData>>);

    use_effect(move || {
        if open() {
            if let Some(container) = container_ref.read().clone() {
                spawn(async move {
                    _ = container.set_focus(true).await;
                });
            }
        }
    });

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
            tabindex: "-1",
            onmounted: move |event| {
                container_ref.set(Some(event.data()));
            },
            onblur: move |_| {
                open.set(false);
            },
            SelectTrigger {
                id: id.clone(),
                open: open(),
                disabled,
                display_text,
                on_toggle: move |_| {
                    if !disabled {
                        open.set(!open());
                    }
                }
            }
            if open() {
                SelectContent {
                    options: options.clone(),
                    selected_value,
                    on_select: move |value: String| {
                        current.set(Some(value.clone()));
                        if let Some(callback) = on_change.as_ref() {
                            callback.call(value);
                        }
                        open.set(false);
                    }
                }
            }
        }
    }
}

#[component]
fn SelectTrigger(
    #[props(into, default)] id: Option<String>,
    open: bool,
    disabled: bool,
    #[props(into)] display_text: String,
    on_toggle: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "ui-select-trigger",
            "data-open": if open { "true" } else { "false" },
            disabled,
            id: id.unwrap_or_default(),
            "aria-haspopup": "listbox",
            "aria-expanded": if open { "true" } else { "false" },
            onclick: move |_| on_toggle.call(()),
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
    }
}

#[component]
fn SelectContent(
    options: Vec<SelectOption>,
    selected_value: Option<String>,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "ui-select-content",
            div {
                class: "ui-select-list",
                for option in options {
                    {
                        let is_active = selected_value.as_ref().map(|v| v == &option.value).unwrap_or(false);
                        rsx! {
                            SelectItem {
                                option,
                                is_active,
                                on_select
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SelectItem(option: SelectOption, is_active: bool, on_select: EventHandler<String>) -> Element {
    let value = option.value.clone();

    rsx! {
        button {
            class: "ui-select-item",
            "data-state": if is_active { "active" } else { "inactive" },
            onmousedown: move |event| {
                event.prevent_default();
                on_select.call(value.clone());
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
