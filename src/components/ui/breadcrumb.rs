use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Crumb {
    pub label: String,
    pub href: Option<String>,
}

impl Crumb {
    pub fn new(label: impl Into<String>, href: Option<impl Into<String>>) -> Self {
        Self {
            label: label.into(),
            href: href.map(|value| value.into()),
        }
    }
}

#[component]
pub fn Breadcrumb(
    #[props(into)] items: Vec<Crumb>,
    #[props(into, default = "/".to_string())] separator: String,
) -> Element {
    rsx! {
        nav {
            role: "navigation",
            aria_label: "Breadcrumb",
            span {
                class: "ui-breadcrumb",
                for (index, item) in items.iter().enumerate() {
                    if let Some(href) = &item.href {
                        a { href: href.clone(), "{item.label}" }
                    } else {
                        span { "{item.label}" }
                    }

                    if index < items.len().saturating_sub(1) {
                        span { class: "ui-breadcrumb-separator", "{separator}" }
                    }
                }
            }
        }
    }
}
