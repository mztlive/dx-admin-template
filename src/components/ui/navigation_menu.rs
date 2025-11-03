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
    let active = use_signal(|| None::<usize>);

    let trigger_nodes: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let mut active_signal = active.clone();
            let is_active = active().is_some_and(|current| current == index);

            rsx! {
                button {
                    class: "ui-navmenu-trigger",
                    "data-open": if is_active { "true" } else { "false" },
                    onmouseenter: move |_| active_signal.set(Some(index)),
                    onclick: move |_| active_signal.set(Some(index)),
                    "{item.label}"
                }
            }
        })
        .collect();

    let selected_content = active()
        .and_then(|index| items.get(index))
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

    let mut reset_active = active.clone();

    rsx! {
        div {
            class: "ui-navmenu",
            onmouseleave: move |_| reset_active.set(None),
            {trigger_nodes.into_iter()}
            if let Some(content) = selected_content {
                {content}
            }
        }
    }
}
