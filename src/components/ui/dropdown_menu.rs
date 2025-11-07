use dioxus::prelude::*;
use super::button::{Button, ButtonVariant, ButtonSize};

#[derive(Clone, PartialEq)]
pub enum DropdownItemVariant {
    Default,
    Destructive,
}

impl DropdownItemVariant {
    fn to_button_variant(&self) -> ButtonVariant {
        match self {
            DropdownItemVariant::Default => ButtonVariant::Ghost,
            DropdownItemVariant::Destructive => ButtonVariant::Destructive,
        }
    }
}

impl Default for DropdownItemVariant {
    fn default() -> Self {
        DropdownItemVariant::Default
    }
}

#[derive(Clone, PartialEq)]
pub struct DropdownMenuItem {
    pub label: String,
    pub value: String,
    pub shortcut: Option<String>,
    pub variant: DropdownItemVariant,
}

impl DropdownMenuItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            shortcut: None,
            variant: DropdownItemVariant::Default,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn destructive(mut self) -> Self {
        self.variant = DropdownItemVariant::Destructive;
        self
    }
}

#[component]
pub fn DropdownMenu(
    #[props(into)] label: String,
    #[props(into)] items: Vec<DropdownMenuItem>,
    #[props(optional)] on_select: Option<EventHandler<String>>,
) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            class: "ui-dropdown",
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                class: "ui-dropdown-trigger",
                on_click: move |_| open.set(!open()),
                "{label}"
                span {
                    style: "margin-left: 0.5rem; font-size: 0.85rem; opacity: 0.7;",
                    "⋮"
                }
            }
            if open() {
                div {
                    class: "ui-dropdown-content",
                    div {
                        class: "ui-dropdown-list",
                        for item in items {
                            DropdownMenuItemButton {
                                item,
                                on_click: move |value: String| {
                                    if let Some(callback) = on_select.as_ref() {
                                        callback.call(value);
                                    }
                                    open.set(false);
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
fn DropdownMenuItemButton(
    item: DropdownMenuItem,
    on_click: EventHandler<String>,
) -> Element {
    let value = item.value.clone();

    rsx! {
        Button {
            variant: item.variant.to_button_variant(),
            size: ButtonSize::Sm,
            class: "ui-dropdown-item",
            on_click: move |_| on_click.call(value.clone()),
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
