use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum DropdownItemVariant {
    Default,
    Destructive,
}

impl DropdownItemVariant {
    fn as_str(&self) -> &'static str {
        match self {
            DropdownItemVariant::Default => "default",
            DropdownItemVariant::Destructive => "destructive",
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
    let open = use_signal(|| false);
    let on_select_handler = on_select.clone();

    rsx! {
        div {
            class: "ui-dropdown",
            button {
                class: "ui-dropdown-trigger",
                "data-open": if open() { "true" } else { "false" },
                onclick: {
                    let mut signal = open.clone();
                    move |_| {
                        let new_state = !signal();
                        signal.set(new_state);
                    }
                },
                span { "{label}" }
                span {
                    style: "font-size: 0.85rem; opacity: 0.7;",
                    "⋮"
                }
            }
            if open() {
                div {
                    class: "ui-dropdown-content",
                    div {
                        class: "ui-dropdown-list",
                        for item in items.iter().cloned() {
                            {
                                let value = item.value.clone();
                                let shortcut = item.shortcut.clone();
                                let variant = item.variant.as_str().to_string();
                                let mut open_signal = open.clone();
                                let handler = on_select_handler.clone();

                                rsx! {
                                    button {
                                        class: "ui-dropdown-item",
                                        "data-variant": variant,
                                        onclick: {
                                            let value = value.clone();
                                            let handler = handler.clone();
                                            move |_| {
                                                if let Some(callback) = handler.clone() {
                                                    callback.call(value.clone());
                                                }
                                                open_signal.set(false);
                                            }
                                        },
                                        span { "{item.label}" }
                                        if let Some(shortcut) = shortcut.clone() {
                                            span { style: "font-size: 0.75rem; opacity: 0.6;", "{shortcut}" }
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
