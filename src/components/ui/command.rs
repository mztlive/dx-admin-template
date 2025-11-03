use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CommandItem {
    pub label: String,
    pub value: String,
    pub shortcut: Option<String>,
    pub group: Option<String>,
}

impl CommandItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            shortcut: None,
            group: None,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }
}

#[component]
pub fn CommandPalette(
    #[props(into)] items: Vec<CommandItem>,
    #[props(optional)] on_select: Option<EventHandler<String>>,
    #[props(into, default = "Search commands".to_string())] placeholder: String,
) -> Element {
    let mut query = use_signal(|| String::new());

    let mut filtered = items.clone();
    let q = query().to_lowercase();
    if !q.is_empty() {
        filtered.retain(|item| {
            item.label.to_lowercase().contains(&q)
                || item
                    .group
                    .as_ref()
                    .map(|g| g.to_lowercase().contains(&q))
                    .unwrap_or(false)
        });
    }

    let handler = on_select.clone();

    let command_nodes: Vec<_> = filtered
        .iter()
        .map(|item| {
            let value = item.value.clone();
            let shortcut = item.shortcut.clone();
            let group = item.group.clone();
            let handler_clone = handler.clone();

            rsx! {
                div {
                    class: "ui-command-item",
                    "data-state": "inactive",
                    onclick: move |_| {
                        if let Some(cb) = handler_clone.clone() {
                            cb.call(value.clone());
                        }
                    },
                    div {
                        style: "display: flex; flex-direction: column; gap: 0.2rem;",
                        span { "{item.label}" }
                        if let Some(group) = group {
                            span { style: "font-size: 0.7rem; color: hsl(var(--muted-foreground));", "{group}" }
                        }
                    }
                    if let Some(shortcut) = shortcut {
                        span { style: "font-size: 0.75rem; opacity: 0.6;", "{shortcut}" }
                    }
                }
            }
        })
        .collect();

    rsx! {
        div {
            class: "ui-command",
            div {
                class: "ui-command-header",
                span { style: "font-size: 0.85rem; opacity: 0.6;", "⌘K" }
                input {
                    class: "ui-command-input",
                    value: query(),
                    placeholder: placeholder.clone(),
                    oninput: move |event| query.set(event.value()),
                }
            }
            div {
                class: "ui-command-list",
                if command_nodes.is_empty() {
                    span { style: "padding: 0.6rem 0.9rem; color: hsl(var(--muted-foreground));", "No results" }
                } else {
                    {command_nodes.into_iter()}
                }
            }
        }
    }
}
