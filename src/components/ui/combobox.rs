use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[derive(Clone, PartialEq)]
pub struct ComboboxOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
}

impl ComboboxOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[component]
pub fn Combobox(
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] id: Option<String>,
    #[props(into)] placeholder: String,
    #[props(into, default)] search_placeholder: Option<String>,
    #[props(into)] options: Vec<ComboboxOption>,
    #[props(into, default)] selected: Option<String>,
    #[props(default)] disabled: bool,
    #[props(optional)] on_select: Option<EventHandler<String>>,
) -> Element {
    let classes = merge_class("ui-combobox", class);
    let trigger_id = id.unwrap_or_default();
    let search_placeholder_text = search_placeholder.unwrap_or_else(|| "Search...".to_string());
    let mut open = use_signal(|| false);
    let mut current_selection = use_signal(move || selected.clone());
    let mut query = use_signal(|| String::new());

    let current_value = current_selection();
    let display_label = current_value
        .as_ref()
        .and_then(|value| {
            options
                .iter()
                .find(|option| option.value == *value)
                .map(|option| option.label.clone())
        })
        .unwrap_or_else(|| placeholder.clone());

    let filtered_options: Vec<ComboboxOption> = {
        let query_text = query().to_lowercase();
        if query_text.is_empty() {
            options.clone()
        } else {
            options
                .iter()
                .cloned()
                .filter(|option| option.label.to_lowercase().contains(&query_text))
                .collect()
        }
    };

    rsx! {
        div {
            class: classes,
            "data-disabled": disabled,
            onfocusout: move |_| open.set(false),
            ComboboxTrigger {
                id: trigger_id,
                disabled,
                open: open(),
                display_label,
                on_toggle: move |_| {
                    if !disabled {
                        let next_state = !open();
                        open.set(next_state);
                        if !next_state {
                            query.set(String::new());
                        }
                    }
                }
            }
            if open() {
                ComboboxContent {
                    search_placeholder: search_placeholder_text,
                    query,
                    filtered_options,
                    current_value,
                    on_select: move |value: String| {
                        current_selection.set(Some(value.clone()));
                        if let Some(callback) = on_select.as_ref() {
                            callback.call(value);
                        }
                        open.set(false);
                        query.set(String::new());
                    }
                }
            }
        }
    }
}

#[component]
fn ComboboxTrigger(
    #[props(into)] id: String,
    disabled: bool,
    open: bool,
    #[props(into)] display_label: String,
    on_toggle: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "ui-combobox-trigger",
            id,
            "aria-haspopup": "dialog",
            "aria-expanded": if open { "true" } else { "false" },
            disabled,
            onclick: move |_| on_toggle.call(()),
            span { "{display_label}" }
            span { class: "ui-combobox-caret", if open { "▲" } else { "▼" } }
        }
    }
}

#[component]
fn ComboboxContent(
    #[props(into)] search_placeholder: String,
    mut query: Signal<String>,
    filtered_options: Vec<ComboboxOption>,
    current_value: Option<String>,
    on_select: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "ui-combobox-content",
            div {
                class: "ui-combobox-search",
                input {
                    class: "ui-combobox-input",
                    placeholder: search_placeholder,
                    r#type: "text",
                    autofocus: true,
                    value: "{query()}",
                    oninput: move |event| query.set(event.value()),
                }
            }
            if filtered_options.is_empty() {
                div {
                    class: "ui-combobox-empty",
                    "No results found"
                }
            } else {
                ul {
                    class: "ui-combobox-list",
                    for option in filtered_options {
                        {
                            let is_active = current_value.as_ref().map(|v| v == &option.value).unwrap_or(false);
                            rsx! {
                                ComboboxItem {
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
}

#[component]
fn ComboboxItem(
    option: ComboboxOption,
    is_active: bool,
    on_select: EventHandler<String>,
) -> Element {
    let value = option.value.clone();

    rsx! {
        li {
            class: "ui-combobox-item",
            "data-state": if is_active { "active" } else { "inactive" },
            button {
                r#type: "button",
                onclick: move |_| on_select.call(value.clone()),
                span { class: "ui-combobox-label", "{option.label}" }
                if let Some(description) = option.description {
                    span { class: "ui-combobox-description", "{description}" }
                }
            }
        }
    }
}
