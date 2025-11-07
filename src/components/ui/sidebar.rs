use dioxus::prelude::*;
use super::utils::{merge_class, data_bool};
#[component]
pub fn Sidebar(
    #[props(default)] collapsed: bool,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-sidebar", class);
    rsx! {
        aside {
            class: classes,
            "data-collapsed": data_bool(collapsed),
            {children}
        }
    }
}

#[component]
pub fn SidebarLayout(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-layout", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarInset(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-inset", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarRail(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-rail", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarHeader(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-header", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarContent(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-content", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarFooter(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-footer", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarSeparator(#[props(into, default)] class: Option<String>) -> Element {
    let classes = merge_class("ui-sidebar-separator", class);
    rsx! {
        div {
            class: classes,
            role: "separator",
        }
    }
}

#[component]
pub fn SidebarGroup(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-group", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarGroupLabel(
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-sidebar-group-label", class);
    rsx! {
        span {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarGroupContent(
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-sidebar-group-content", class);
    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarMenu(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-sidebar-menu", class);
    rsx! {
        nav {
            class: classes,
            ul {
                class: "ui-sidebar-menu-list",
                {children}
            }
        }
    }
}

#[component]
pub fn SidebarMenuItem(
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-sidebar-menu-item", class);
    rsx! {
        li {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn SidebarMenuButton(
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    #[props(into, default)] badge: Option<String>,
    #[props(into, default)] icon: Option<String>,
    #[props(into, default)] href: Option<String>,
    #[props(default)] active: bool,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_click: Option<EventHandler<MouseEvent>>,
) -> Element {
    let classes = merge_class("ui-sidebar-menu-button", class);
    let description_text = description.clone();
    let badge_text = badge.clone();
    let icon_text = icon
        .clone()
        .or_else(|| label.chars().next().map(|character| character.to_string()));

    let handler_for_link = on_click.clone();
    let handler_for_button = on_click.clone();

    let label_clone = label.clone();

    if let Some(href_value) = href.clone() {
        rsx! {
            a {
                class: classes,
                href: href_value,
                "data-active": data_bool(active),
                onclick: move |event| {
                    if let Some(handler) = handler_for_link.clone() {
                        handler.call(event);
                    }
                },
                span { class: "ui-sidebar-button-body",
                    span {
                        class: "ui-sidebar-icon",
                        if let Some(icon_value) = icon_text.clone() {
                            "{icon_value}"
                        }
                    }
                    span {
                        class: "ui-sidebar-text",
                        span { class: "ui-sidebar-label", "{label_clone}" }
                        if let Some(description_value) = description_text.clone() {
                            span {
                                class: "ui-sidebar-description",
                                "{description_value}"
                            }
                        }
                    }
                    if let Some(badge_value) = badge_text.clone() {
                        span {
                            class: "ui-sidebar-badge",
                            "{badge_value}"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            button {
                class: classes,
                "data-active": data_bool(active),
                r#type: "button",
                onclick: move |event| {
                    if let Some(handler) = handler_for_button.clone() {
                        handler.call(event);
                    }
                },
                span { class: "ui-sidebar-button-body",
                    span {
                        class: "ui-sidebar-icon",
                        if let Some(icon_value) = icon_text.clone() {
                            "{icon_value}"
                        }
                    }
                    span {
                        class: "ui-sidebar-text",
                        span { class: "ui-sidebar-label", "{label}" }
                        if let Some(description_value) = description_text.clone() {
                            span {
                                class: "ui-sidebar-description",
                                "{description_value}"
                            }
                        }
                    }
                    if let Some(badge_value) = badge_text.clone() {
                        span {
                            class: "ui-sidebar-badge",
                            "{badge_value}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SidebarMenuBadge(
    #[props(into)] text: String,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let classes = merge_class("ui-sidebar-badge", class);
    rsx! {
        span {
            class: classes,
            "{text}"
        }
    }
}

#[component]
pub fn SidebarTrigger(
    #[props(default)] collapsed: bool,
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] label: Option<String>,
    #[props(optional)] on_toggle: Option<EventHandler<bool>>,
) -> Element {
    let classes = merge_class("ui-sidebar-trigger", class);
    let label_text = label.unwrap_or_else(|| {
        if collapsed {
            "展开侧边栏".to_string()
        } else {
            "收起侧边栏".to_string()
        }
    });
    let state = !collapsed;
    let handler = on_toggle.clone();

    rsx! {
        button {
            class: classes,
            r#type: "button",
            "aria-pressed": data_bool(!collapsed),
            onclick: move |_| {
                if let Some(handler) = handler.clone() {
                    handler.call(state);
                }
            },
            span { class: "ui-sidebar-trigger-icon", if collapsed { "→" } else { "←" } }
            span { class: "ui-sidebar-trigger-label", "{label_text}" }
        }
    }
}
