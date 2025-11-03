use crate::{
    components::ui::{
        Avatar, Button, ButtonVariant, Sidebar, SidebarContent, SidebarFooter, SidebarGroup,
        SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarLayout,
        SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarRail, SidebarSeparator,
    },
    Route,
};
use dioxus::prelude::*;

fn page_title(route: &Route) -> &'static str {
    match route {
        Route::Home {} => "Dashboard overview",
        Route::Components {} => "Component library",
    }
}

#[component]
pub fn Navbar() -> Element {
    let current_route: Route = use_route();
    let is_dark = use_signal(|| false);
    let shell_class = if is_dark() {
        "ui-shell shadcn dark"
    } else {
        "ui-shell shadcn"
    };

    let title = page_title(&current_route);
    let is_dashboard = matches!(current_route, Route::Home { .. });
    let is_components = matches!(current_route, Route::Components { .. });

    let theme_label = {
        let is_dark = is_dark.clone();
        move || {
            if is_dark() {
                "Light mode"
            } else {
                "Dark mode"
            }
        }
    };
    let theme_toggle = is_dark.clone();

    rsx! {
        section {
            class: shell_class,
            SidebarLayout {
                class: "admin-shell",
                SidebarRail {}
                Sidebar {
                    SidebarHeader {
                        div { class: "sidebar-brand",
                            span { class: "sidebar-logo", "⚡" }
                            div {
                                span { class: "sidebar-name", "Dioxus Admin" }
                                span { class: "sidebar-subtitle", "v0.7 toolkit" }
                            }
                        }
                    }
                    SidebarContent {
                        SidebarGroup {
                            SidebarGroupLabel { "Overview" }
                            SidebarGroupContent {
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            label: "Dashboard",
                                            description: Some("KPIs, monitors, and recent activity".to_string()),
                                            icon: Some("📊".to_string()),
                                            active: is_dashboard,
                                            href: Some(Route::Home {}.to_string()),
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            label: "Components",
                                            description: Some("Living style guide of primitives".to_string()),
                                            icon: Some("🧩".to_string()),
                                            active: is_components,
                                            href: Some(Route::Components {}.to_string()),
                                        }
                                    }
                                }
                            }
                        }
                        SidebarSeparator {}
                        SidebarGroup {
                            SidebarGroupLabel { "Shortcuts" }
                            SidebarGroupContent {
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            label: "Team",
                                            description: Some("Invite and manage collaborators".to_string()),
                                            icon: Some("👥".to_string()),
                                            badge: Some("4 pending".to_string()),
                                            href: Some("#team".to_string()),
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            label: "Settings",
                                            description: Some("Branding, auth, billing".to_string()),
                                            icon: Some("⚙️".to_string()),
                                            href: Some("#settings".to_string()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                    SidebarFooter {
                        div { class: "sidebar-profile",
                            Avatar {
                                src: Some("https://avatars.githubusercontent.com/u/3236120?v=4".to_string()),
                                alt: Some("Administrator avatar".to_string()),
                                fallback: Some("DX".to_string()),
                            }
                            div {
                                span { class: "sidebar-profile-name", "Taylor Chen" }
                                span { class: "sidebar-profile-role", "Product Manager" }
                            }
                        }
                        Button {
                            class: Some("mt-2 w-full".to_string()),
                            variant: ButtonVariant::Secondary,
                            r#type: "button".to_string(),
                            "Switch account"
                        }
                    }
                }
                SidebarInset {
                    class: "admin-shell-inset",
                    header {
                        class: "admin-shell-topbar",
                        div { class: "admin-shell-command",
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: Some("admin-command-button".to_string()),
                                r#type: "button".to_string(),
                                "⌘K"
                            }
                        }
                        div { class: "admin-shell-meta",
                            h1 { class: "admin-shell-title", "{title}" }
                        }
                        div { class: "admin-shell-actions",
                            Button {
                                variant: ButtonVariant::Secondary,
                                class: Some("admin-shell-theme".to_string()),
                                r#type: "button".to_string(),
                                on_click: move |_| {
                                    let mut handle = theme_toggle.clone();
                                    let current = handle();
                                    handle.set(!current);
                                },
                                "{theme_label()}"
                            }
                        }
                    }
                    main {
                        class: "admin-shell-content",
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}
