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
    let search_placeholder = search_placeholder.unwrap_or_else(|| "Search...".to_string());
    let open = use_signal(|| false);
    let current_selection = use_signal(move || selected.clone());
    let query = use_signal(|| String::new());
    let on_select_handler = on_select.clone();

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
            onfocusout: {
                let mut open_signal = open.clone();
                move |_| open_signal.set(false)
            },
            button {
                class: "ui-combobox-trigger",
                id: trigger_id.clone(),
                "aria-haspopup": "dialog",
                "aria-expanded": if open() { "true" } else { "false" },
                disabled,
                onclick: {
                    let mut open_signal = open.clone();
                    move |_| {
                        if !disabled {
                            let next_state = !open_signal();
                            open_signal.set(next_state);
                            if !next_state {
                                let mut query_signal = query.clone();
                                query_signal.set(String::new());
                            }
                        }
                    }
                },
                span { "{display_label}" }
                span { class: "ui-combobox-caret", if open() { "▲" } else { "▼" } }
            }
            if open() {
                div {
                    class: "ui-combobox-content",
                    div {
                        class: "ui-combobox-search",
                        input {
                            class: "ui-combobox-input",
                            placeholder: search_placeholder.clone(),
                            r#type: "text",
                            autofocus: true,
                            value: "{query()}",
                            oninput: {
                                let mut query_signal = query.clone();
                                move |event| query_signal.set(event.value())
                            },
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
                                    let is_active = current_value
                                        .as_ref()
                                        .map(|value| value == &option.value)
                                        .unwrap_or(false);
                                    let option_value = option.value.clone();
                                    let option_label = option.label.clone();
                                    let option_description = option.description.clone();
                                    rsx! {
                                        li {
                                            class: "ui-combobox-item",
                                            "data-state": if is_active { "active" } else { "inactive" },
                                            button {
                                                r#type: "button",
                                                onclick: {
                                                    let option_value = option_value.clone();
                                                    let handler = on_select_handler.clone();
                                                    let mut open_signal = open.clone();
                                                    let mut current_signal = current_selection.clone();
                                                    let mut query_signal = query.clone();
                                                    move |_| {
                                                        current_signal.set(Some(option_value.clone()));
                                                        if let Some(callback) = handler.clone() {
                                                            callback.call(option_value.clone());
                                                        }
                                                        open_signal.set(false);
                                                        query_signal.set(String::new());
                                                    }
                                                },
                                                span { class: "ui-combobox-label", "{option_label}" }
                                                if let Some(description) = option_description {
                                                    span { class: "ui-combobox-description", "{description}" }
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
}
