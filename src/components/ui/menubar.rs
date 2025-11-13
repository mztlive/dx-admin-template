use super::button::{Button, ButtonSize, ButtonVariant};
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

    rsx! {
        div {
            class: "ui-menubar",
            onmouseleave: move |_| open.set(None),
            for (index, menu) in menus.into_iter().enumerate() {
                MenubarMenuTrigger {
                    menu,
                    index,
                    is_open: open() == Some(index),
                    on_hover: move |idx| open.set(Some(idx)),
                    on_click: move |idx| open.set(Some(idx)),
                    on_item_select: move |value: String| {
                        if let Some(cb) = on_select.as_ref() {
                            cb.call(value);
                        }
                        open.set(None);
                    }
                }
            }
        }
    }
}

#[component]
fn MenubarMenuTrigger(
    menu: MenubarMenu,
    index: usize,
    is_open: bool,
    on_hover: EventHandler<usize>,
    on_click: EventHandler<usize>,
    on_item_select: EventHandler<String>,
) -> Element {
    rsx! {
        span {
            style: "position: relative;",
            onmouseenter: move |_| on_hover.call(index),
            Button {
                variant: if is_open { ButtonVariant::Ghost } else { ButtonVariant::Ghost },
                size: ButtonSize::Sm,
                class: "ui-menubar-trigger",
                on_click: move |_| on_click.call(index),
                "{menu.label}"
            }
            if is_open {
                div {
                    class: "ui-menubar-content",
                    for item in menu.items {
                        MenubarItemButton {
                            item,
                            on_select: on_item_select
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MenubarItemButton(item: MenubarItem, on_select: EventHandler<String>) -> Element {
    let value = item.value.clone();
    let variant = if item.destructive {
        ButtonVariant::Destructive
    } else {
        ButtonVariant::Ghost
    };

    rsx! {
        Button {
            variant,
            size: ButtonSize::Sm,
            class: "ui-menubar-item",
            on_click: move |_| on_select.call(value.clone()),
            span { "{item.label}" }
            if let Some(shortcut) = item.shortcut {
                span {
                    style: "margin-left: auto; font-size: 0.75rem; opacity: 0.6;",
                    "{shortcut}"
                }
            }
        }
    }
}
