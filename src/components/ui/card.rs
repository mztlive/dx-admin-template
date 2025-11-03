use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[component]
pub fn Card(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-card", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn CardHeader(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-card-header", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-card-title", class);
    rsx! {
        h3 {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-card-description", class);
    rsx! {
        p {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn CardContent(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-card-content", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-card-footer", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}
