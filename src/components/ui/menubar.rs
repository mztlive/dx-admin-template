use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MenubarItem {
    pub label: String,
    pub value: String,
    pub shortcut: Option<String>,
    pub destructive: bool,
}

impl MenubarItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            shortcut: None,
            destructive: false,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

#[derive(Clone, PartialEq)]
pub struct MenubarMenu {
    pub label: String,
    pub items: Vec<MenubarItem>,
}

impl MenubarMenu {
    pub fn new(label: impl Into<String>, items: Vec<MenubarItem>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}

#[component]
pub fn Menubar(
    #[props(into)] menus: Vec<MenubarMenu>,
    #[props(optional)] on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| None::<usize>);
    let handler = on_select.clone();

    let menu_nodes: Vec<_> = menus
        .iter()
        .enumerate()
        .map(|(index, menu)| {
            let mut open_signal_hover = open.clone();
            let mut open_signal_click = open.clone();
            let is_open = open() == Some(index);
            let item_nodes: Vec<_> = menu
                .items
                .iter()
                .map(|item| {
                    let value = item.value.clone();
                    let shortcut = item.shortcut.clone();
                    let destructive = item.destructive;
                    let handler_clone = handler.clone();
                    let mut open_close = open.clone();

                    rsx! {
                        button {
                            class: "ui-menubar-item",
                            "data-variant": if destructive { "destructive" } else { "default" },
                            onclick: move |_| {
                                if let Some(cb) = handler_clone.clone() {
                                    cb.call(value.clone());
                                }
                                open_close.set(None);
                            },
                            span { "{item.label}" }
                            if let Some(shortcut) = shortcut.clone() {
                                span { style: "font-size: 0.75rem; opacity: 0.6;", "{shortcut}" }
                            }
                        }
                    }
                })
                .collect();

            rsx! {
                span {
                    style: "position: relative;",
                    button {
                        class: "ui-menubar-trigger",
                        "data-open": if is_open { "true" } else { "false" },
                        onmouseenter: move |_| open_signal_hover.set(Some(index)),
                        onclick: move |_| open_signal_click.set(Some(index)),
                        "{menu.label}"
                    }
                    if is_open {
                        div { class: "ui-menubar-content", {item_nodes.into_iter()} }
                    }
                }
            }
        })
        .collect();

    rsx! {
        div {
            class: "ui-menubar",
            onmouseleave: move |_| open.set(None),
                {menu_nodes.into_iter()}
        }
    }
}
