use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NavigationItem {
    pub label: String,
    pub description: Option<String>,
    pub href: String,
}

impl NavigationItem {
    pub fn new(
        label: impl Into<String>,
        href: impl Into<String>,
        description: Option<impl Into<String>>,
    ) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            description: description.map(|d| d.into()),
        }
    }
}

#[component]
pub fn NavigationMenu(#[props(into)] items: Vec<NavigationItem>) -> Element {
    let active = use_signal(|| 0usize);

    let trigger_nodes: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let mut active_signal = active.clone();
            let is_active = active() == index;

            rsx! {
                button {
                    class: "ui-navmenu-trigger",
                    "data-open": if is_active { "true" } else { "false" },
                    onmouseenter: move |_| active_signal.set(index),
                    onclick: move |_| active_signal.set(index),
                    "{item.label}"
                }
            }
        })
        .collect();

    let selected_content = items
        .get(active().min(items.len().saturating_sub(1)))
        .map(|item| {
            let description = item.description.clone();
            rsx! {
                div {
                    class: "ui-navmenu-content",
                    a {
                        href: item.href.clone(),
                        class: "ui-navmenu-item",
                        "{item.label}"
                    }
                    if let Some(desc) = description {
                        span { style: "font-size: 0.75rem; color: hsl(var(--muted-foreground));", "{desc}" }
                    }
                }
            }
        });

    rsx! {
        div {
            class: "ui-navmenu",
            {trigger_nodes.into_iter()}
            if let Some(content) = selected_content {
                {content}
            }
        }
    }
}
