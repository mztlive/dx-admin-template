use dioxus::prelude::*;

/// Visual variants for alerts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertVariant {
    Default,
    Destructive,
}

impl AlertVariant {
    fn as_str(&self) -> &'static str {
        match self {
            AlertVariant::Default => "default",
            AlertVariant::Destructive => "destructive",
        }
    }
}

impl Default for AlertVariant {
    fn default() -> Self {
        AlertVariant::Default
    }
}

#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(into, default)] title: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "ui-alert",
            "data-variant": variant.as_str(),
            if let Some(title) = title {
                h4 { class: "ui-alert-title", "{title}" }
            }
            div {
                class: "ui-alert-description",
                {children}
            }
        }
    }
}
