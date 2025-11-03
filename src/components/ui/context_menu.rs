use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ContextItem {
    pub label: String,
    pub value: String,
    pub destructive: bool,
}

impl ContextItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            destructive: false,
        }
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

#[component]
pub fn ContextMenu(
    #[props(into)] items: Vec<ContextItem>,
    #[props(optional)] on_select: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let mut position = use_signal(|| None::<(f32, f32)>);
    let handler = on_select.clone();

    let menu_portal = position().map(|(x, y)| {
        let nodes: Vec<_> = items
            .iter()
            .map(|item| {
                let value = item.value.clone();
                let mut pos_signal = position.clone();
                let handler_clone = handler.clone();

                rsx! {
                    button {
                        class: "ui-context-item",
                        "data-variant": if item.destructive { "destructive" } else { "default" },
                        onclick: move |_| {
                            if let Some(cb) = handler_clone.clone() {
                                cb.call(value.clone());
                            }
                            pos_signal.set(None);
                        },
                        "{item.label}"
                    }
                }
            })
            .collect();
        (x, y, nodes)
    });

    rsx! {
        span {
            class: "ui-context-trigger",
            oncontextmenu: move |event| {
                event.prevent_default();
                let coords = event.client_coordinates();
                position.set(Some((coords.x as f32, coords.y as f32)));
            },
            {children}
        }
        if let Some((x, y, nodes)) = menu_portal {
            div {
                style: "position: fixed; inset: 0; z-index: 39;",
                onclick: move |_| position.set(None),
            }
            div {
                class: "ui-context-menu",
                style: format!("top: {y}px; left: {x}px;"),
                {nodes.into_iter()}
            }
        }
    }
}
