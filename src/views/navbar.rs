use crate::{
    components::ui::{
        Avatar, Badge, BadgeVariant, Button, ButtonVariant, Sidebar, SidebarContent, SidebarFooter,
        SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset,
        SidebarLayout, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarRail,
        SidebarSeparator, SidebarTrigger,
    },
    Route,
};
use dioxus::prelude::*;

fn page_meta(route: &Route) -> (&'static str, &'static str) {
    match route {
        Route::Home {} => (
            "Dashboard overview",
            "Track product health, monitor key funnels, and coordinate the team from one place.",
        ),
        Route::Components {} => (
            "Component library",
            "Browse every primitive wired into this starter so new views stay consistent.",
        ),
    }
}

#[component]
pub fn Navbar() -> Element {
    let collapsed = use_signal(|| false);
    let current_route: Route = use_route();
    let collapsed_state = collapsed();
    let collapsed_setter = collapsed.clone();

    let (page_title, page_description) = page_meta(&current_route);
    let is_dashboard = matches!(current_route, Route::Home { .. });
    let is_components = matches!(current_route, Route::Components { .. });

    rsx! {
        section {
            class: "ui-shell shadcn",
            SidebarLayout {
                class: "admin-shell",
                SidebarRail { }
                Sidebar {
                    collapsed: collapsed_state,
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
                        SidebarSeparator { }
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
                        SidebarTrigger {
                            collapsed: collapsed_state,
                            on_toggle: move |next_collapsed: bool| {
                                collapsed_setter.clone().set(next_collapsed);
                            },
                        }
                        div { class: "admin-shell-meta",
                            h1 { class: "admin-shell-title", "{page_title}" }
                            p { class: "admin-shell-description", "{page_description}" }
                        }
                        div { class: "admin-shell-actions",
                            Badge { variant: BadgeVariant::Secondary, "Operational" }
                            Button {
                                variant: ButtonVariant::Default,
                                class: Some("admin-shell-report".to_string()),
                                r#type: "button".to_string(),
                                "New report"
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
