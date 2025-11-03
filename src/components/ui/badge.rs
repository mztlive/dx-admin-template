use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Outline,
    Destructive,
}

impl BadgeVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "default",
            BadgeVariant::Secondary => "secondary",
            BadgeVariant::Outline => "outline",
            BadgeVariant::Destructive => "destructive",
        }
    }
}

impl Default for BadgeVariant {
    fn default() -> Self {
        BadgeVariant::Default
    }
}

#[component]
pub fn Badge(
    #[props(default)] variant: BadgeVariant,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let mut classes = String::from("ui-badge");
    if let Some(extra) = class.filter(|extra| !extra.trim().is_empty()) {
        classes.push(' ');
        classes.push_str(extra.trim());
    }

    rsx! {
        span {
            class: classes,
            "data-variant": variant.as_str(),
            {children}
        }
    }
}
