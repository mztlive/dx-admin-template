use crate::components::{
    ui::{
        Accordion, AccordionContent, AccordionItem, AccordionTrigger, Alert, AlertVariant, Avatar,
        Badge, BadgeVariant, Breadcrumb, Button, ButtonSize, ButtonVariant, Card, CardContent,
        CardDescription, CardFooter, CardHeader, CardTitle, Checkbox, CommandItem, CommandPalette,
        ContextItem, ContextMenu, Crumb, Dialog, DropdownMenu, DropdownMenuItem, HoverCard, Input,
        Label, Menubar, MenubarItem, MenubarMenu, NavigationItem, NavigationMenu, Pagination,
        Popover, Progress, RadioGroup, RadioGroupItem, Select, SelectOption, Separator,
        SeparatorOrientation, Sheet, SheetSide, Slider, StepItem, Steps, Switch, Tabs, TabsContent,
        TabsList, TabsTrigger, Textarea, Toast, ToastViewport, Tooltip,
    },
    Echo, Hero,
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
        UiShowcase {}
    }
}

#[component]
fn UiShowcase() -> Element {
    let accepted_terms = use_signal(|| false);
    let email_notifications = use_signal(|| true);
    let slider_value = use_signal(|| 42.0f32);
    let contact_method = use_signal(|| "email".to_string());
    let newsletter_opt_in = use_signal(|| true);
    let dark_mode = use_signal(|| false);
    let theme_choice = use_signal(|| Some("system".to_string()));
    let menu_selection = use_signal(|| "Select a menu action".to_string());
    let menubar_selection = use_signal(|| "Choose a menu item".to_string());
    let pagination_current = use_signal(|| 3usize);
    let steps_current = use_signal(|| 2usize);
    let command_selection = use_signal(|| "Nothing selected yet".to_string());
    let context_selection = use_signal(|| "Right click the area to choose an action".to_string());
    let dialog_open = use_signal(|| false);
    let sheet_open = use_signal(|| false);
    let toast_open = use_signal(|| false);
    let slider_value_signal = slider_value.clone();
    let slider_value_setter = slider_value.clone();
    let contact_method_signal = contact_method.clone();
    let theme_choice_signal = theme_choice.clone();
    let accepted_terms_setter = accepted_terms.clone();
    let email_notifications_setter = email_notifications.clone();
    let contact_method_setter = contact_method.clone();
    let newsletter_opt_in_setter = newsletter_opt_in.clone();
    let dark_mode_setter = dark_mode.clone();
    let theme_choice_setter = theme_choice.clone();
    let menu_selection_setter = menu_selection.clone();
    let menubar_selection_setter = menubar_selection.clone();
    let pagination_setter = pagination_current.clone();
    let steps_setter = steps_current.clone();
    let command_selection_setter = command_selection.clone();
    let context_selection_setter = context_selection.clone();
    let dialog_signal = dialog_open.clone();
    let sheet_signal = sheet_open.clone();
    let toast_signal = toast_open.clone();
    let intensity_text = move || format!("Accent intensity: {:.0}%", slider_value_signal());
    let contact_text = move || format!("Preferred contact: {}", contact_method_signal());
    let select_options = vec![
        SelectOption::new("System", "system"),
        SelectOption::new("Light", "light"),
        SelectOption::new("Dark", "dark"),
    ];
    let menu_items = vec![
        DropdownMenuItem::new("Profile", "profile").with_shortcut("⌘P"),
        DropdownMenuItem::new("Billing", "billing").with_shortcut("⌘B"),
        DropdownMenuItem::new("Team", "team"),
        DropdownMenuItem::new("Sign out", "logout").destructive(),
    ];
    let breadcrumb_items = vec![
        Crumb::new("Dashboard", Some("#")),
        Crumb::new("Settings", Some("#settings")),
        Crumb::new("Team", None::<String>),
    ];
    let navigation_items = vec![
        NavigationItem::new(
            "Overview",
            "#overview",
            Some("Project snapshots and quick metrics"),
        ),
        NavigationItem::new(
            "Playground",
            "#playground",
            Some("Prototype new ideas and components"),
        ),
        NavigationItem::new(
            "Documentation",
            "https://dioxuslabs.com/learn",
            Some("Dive into the latest Dioxus 0.7 docs"),
        ),
    ];
    let menubar_menus = vec![
        MenubarMenu::new(
            "File",
            vec![
                MenubarItem::new("New Tab", "new_tab").shortcut("⌘T"),
                MenubarItem::new("Open Workspace", "open_workspace"),
                MenubarItem::new("Save", "save").shortcut("⌘S"),
            ],
        ),
        MenubarMenu::new(
            "Edit",
            vec![
                MenubarItem::new("Undo", "undo").shortcut("⌘Z"),
                MenubarItem::new("Redo", "redo").shortcut("⇧⌘Z"),
                MenubarItem::new("Delete", "delete").destructive(),
            ],
        ),
    ];
    let command_items = vec![
        CommandItem::new("Create project", "create_project")
            .shortcut("⌘N")
            .group("Actions"),
        CommandItem::new("Invite teammate", "invite").group("Actions"),
        CommandItem::new("Open documentation", "docs").group("Resources"),
        CommandItem::new("Keyboard shortcuts", "shortcuts").group("Resources"),
    ];
    let context_items = vec![
        ContextItem::new("Rename", "rename"),
        ContextItem::new("Duplicate", "duplicate"),
        ContextItem::new("Archive", "archive"),
        ContextItem::new("Delete", "delete").destructive(),
    ];
    let steps_items = vec![
        StepItem::new("Plan", Some("Outline requirements")),
        StepItem::new("Build", Some("Implement features")),
        StepItem::new("Review", Some("QA and ship")),
    ];
    let total_pages = 8usize;
    let pagination_summary =
        move || format!("Showing page {} of {total_pages}", pagination_current());
    let steps_total = steps_items.len();
    let steps_summary = move || format!("Stage {} of {steps_total}", steps_current());
    let theme_display = {
        let current = theme_choice();
        current
            .as_ref()
            .and_then(|value| {
                select_options
                    .iter()
                    .find(|option| option.value == *value)
                    .map(|option| option.label.clone())
            })
            .unwrap_or_else(|| "System".to_string())
    };
    let theme_summary = format!("Active theme: {theme_display}");

    rsx! {
        section {
            class: if dark_mode() { "ui-shell shadcn dark" } else { "ui-shell shadcn" },
            "data-theme": if dark_mode() { "dark" } else { "light" },
            div {
                class: "ui-stack",
                h2 { style: "font-size: 1.75rem; font-weight: 600;", "Shadcn primitives for Dioxus" }
                p {
                    style: "color: hsl(var(--muted-foreground)); max-width: 640px;",
                    "A compact gallery of the shadcn/ui building blocks, rebuilt with Dioxus 0.7 signals."
                }
            }
            div {
                class: "ui-demo-grid",

                Card {
                    CardHeader {
                        CardTitle { "Profile form" }
                        CardDescription { "Inputs, sliders, helpers, and actions inside a card layout." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            Label { html_for: "profile-name", "Name" }
                            Input { id: "profile-name", placeholder: "Ada Lovelace" }
                        }
                        div { class: "ui-stack",
                            Label { html_for: "profile-about", "About" }
                            Textarea {
                                id: "profile-about",
                                placeholder: "Tell us something fun...",
                                rows: 4,
                            }
                            SpanHelper { "Textarea adopts shadcn spacing and typography out of the box." }
                        }
                        Separator { style: "margin: 1rem 0;" }
                        div { class: "ui-stack",
                            Label { html_for: "accent-slider", "Accent strength" }
                            Slider {
                                value: slider_value(),
                                min: 0.0,
                                max: 100.0,
                                step: 1.0,
                                on_value_change: {
                                    let mut signal = slider_value_setter.clone();
                                    move |val| signal.set(val)
                                },
                            }
                            Progress { value: slider_value(), max: 100.0 }
                            SpanHelper { "{intensity_text()}" }
                        }
                        div { class: "ui-stack",
                            Label { html_for: "theme-select", "Theme preference" }
                            Select {
                                id: Some("theme-select".to_string()),
                                placeholder: "Select a theme",
                                options: select_options.clone(),
                                selected: theme_choice_signal(),
                                on_change: move |value| {
                                    let mut signal = theme_choice_setter.clone();
                                    signal.set(Some(value));
                                },
                            }
                            SpanHelper { "{theme_summary}" }
                        }
                        div { class: "ui-bleed",
                                div { class: "ui-cluster",
                                    Checkbox {
                                        id: Some("accept-terms".to_string()),
                                        checked: accepted_terms(),
                                        on_checked_change: move |state| accepted_terms_setter.clone().set(state),
                                    }
                                    Label { html_for: "accept-terms", "Agree to terms" }
                                }
                                div { class: "ui-cluster",
                                    Label { html_for: "profile-emails", "Email notifications" }
                                    Switch {
                                        id: Some("profile-emails".to_string()),
                                        checked: email_notifications(),
                                        on_checked_change: move |state| email_notifications_setter.clone().set(state),
                                    }
                                }
                            }
                    }
                    CardFooter {
                        div { class: "ui-cluster",
                            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Cancel" }
                            Button { disabled: !accepted_terms(), "Save changes" }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Buttons & badges" }
                        CardDescription { "Variant + size matrix copied directly from shadcn/ui." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            SpanHelper { "Buttons – variants" }
                            div { class: "ui-cluster",
                                Button { "Primary" }
                                Button { variant: ButtonVariant::Secondary, "Secondary" }
                                Button { variant: ButtonVariant::Destructive, "Destructive" }
                                Button { variant: ButtonVariant::Outline, "Outline" }
                                Button { variant: ButtonVariant::Ghost, "Ghost" }
                                Button { variant: ButtonVariant::Link, "Learn more" }
                            }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Buttons – sizes" }
                            div { class: "ui-cluster",
                                Button { size: ButtonSize::Sm, "Small" }
                                Button { "Default" }
                                Button { size: ButtonSize::Lg, "Large" }
                                Button { size: ButtonSize::Icon, "★" }
                            }
                        }
                        Separator { style: "margin: 1rem 0;" }
                        div { class: "ui-stack",
                            SpanHelper { "Badges" }
                            div { class: "ui-cluster",
                                Badge { "Default" }
                                Badge { variant: BadgeVariant::Secondary, "Secondary" }
                                Badge { variant: BadgeVariant::Destructive, "Destructive" }
                                Separator { orientation: SeparatorOrientation::Vertical, style: "height: 1.5rem;" }
                                Badge { variant: BadgeVariant::Outline, "Outline" }
                            }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Select & dropdowns" }
                        CardDescription { "Select, dropdown menu, tooltip and dynamic feedback." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            Label { html_for: "quick-theme", "Quick theme" }
                            Select {
                                id: Some("quick-theme".to_string()),
                                placeholder: "Choose theme",
                                options: select_options.clone(),
                                selected: theme_choice_signal(),
                                on_change: move |value| {
                                    let mut signal = theme_choice_setter.clone();
                                    signal.set(Some(value));
                                },
                            }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Dropdown menu" }
                            DropdownMenu {
                                label: "Open menu",
                                items: menu_items.clone(),
                                on_select: move |value| {
                                    let mut signal = menu_selection_setter.clone();
                                    signal.set(format!("Selected action: {value}"));
                                },
                            }
                            SpanHelper { "{menu_selection()}" }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Tooltip" }
                            Tooltip {
                                label: "Invite collaborators",
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Sm,
                                    "Hover me"
                                }
                            }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Navigation patterns" }
                        CardDescription { "Breadcrumbs, menus, pagination, and progress steps." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            SpanHelper { "Breadcrumb" }
                            Breadcrumb { items: breadcrumb_items.clone(), separator: ">".to_string() }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Navigation menu" }
                            NavigationMenu { items: navigation_items.clone() }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Menubar" }
                            Menubar {
                                menus: menubar_menus.clone(),
                                on_select: move |value| {
                                    let mut signal = menubar_selection_setter.clone();
                                    signal.set(format!("Menubar selected: {value}"));
                                },
                            }
                            SpanHelper { "{menubar_selection()}" }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Pagination" }
                            Pagination {
                                total_pages: total_pages,
                                current_page: pagination_current(),
                                on_page_change: move |page| {
                                    let mut signal = pagination_setter.clone();
                                    signal.set(page);
                                },
                            }
                            SpanHelper { "{pagination_summary()}" }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Steps" }
                            Steps {
                                steps: steps_items.clone(),
                                current: steps_current(),
                            }
                            div { class: "ui-cluster",
                                Button {
                                    variant: ButtonVariant::Outline,
                                    size: ButtonSize::Sm,
                                    on_click: move |_| {
                                        let mut signal = steps_setter.clone();
                                        let prev = signal().saturating_sub(1).max(1);
                                        signal.set(prev);
                                    },
                                    "Previous"
                                }
                                Button {
                                    size: ButtonSize::Sm,
                                    on_click: move |_| {
                                        let mut signal = steps_setter.clone();
                                        let next = (signal() + 1).min(steps_total);
                                        signal.set(next);
                                    },
                                    "Next"
                                }
                            }
                            SpanHelper { "{steps_summary()}" }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Selection controls" }
                        CardDescription { "Checkboxes, switches, and radio groups stay in sync with signals." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            div { class: "ui-cluster",
                                Checkbox {
                                    id: Some("newsletter-opt".to_string()),
                                    checked: newsletter_opt_in(),
                                    on_checked_change: move |state| newsletter_opt_in_setter.clone().set(state),
                                }
                                Label { html_for: "newsletter-opt", "Subscribe to newsletter" }
                            }
                            div { class: "ui-cluster",
                                Label { html_for: "dark-mode", "Dark mode" }
                                Switch {
                                    id: Some("dark-mode".to_string()),
                                    checked: dark_mode(),
                                    on_checked_change: move |state| dark_mode_setter.clone().set(state),
                                }
                            }
                            Separator { style: "margin: 0.75rem 0;" }
                            RadioGroup {
                                default_value: contact_method(),
                                on_value_change: move |value| contact_method_setter.clone().set(value),
                                div { class: "ui-stack",
                                    div { class: "ui-cluster",
                                        RadioGroupItem { id: Some("contact-email".to_string()), value: "email" }
                                        Label { html_for: "contact-email", "Email" }
                                    }
                                    div { class: "ui-cluster",
                                        RadioGroupItem { id: Some("contact-sms".to_string()), value: "sms" }
                                        Label { html_for: "contact-sms", "SMS" }
                                    }
                                    div { class: "ui-cluster",
                                        RadioGroupItem { id: Some("contact-call".to_string()), value: "call" }
                                        Label { html_for: "contact-call", "Phone call" }
                                    }
                                }
                            }
                            SpanHelper { "{contact_text()}" }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Command & context" }
                        CardDescription { "Command palette filtering and contextual menus." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            SpanHelper { "Command palette" }
                            CommandPalette {
                                items: command_items.clone(),
                                on_select: move |value| {
                                    let mut signal = command_selection_setter.clone();
                                    signal.set(format!("Command selected: {value}"));
                                },
                            }
                            SpanHelper { "{command_selection()}" }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Context menu" }
                            ContextMenu {
                                items: context_items.clone(),
                                on_select: move |value| {
                                    let mut signal = context_selection_setter.clone();
                                    signal.set(format!("Context action: {value}"));
                                },
                                div {
                                    style: "padding: 1.5rem; border: 1px dashed hsl(var(--border)); border-radius: var(--radius); text-align: center;",
                                    "Right click anywhere in this box"
                                }
                            }
                            SpanHelper { "{context_selection()}" }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Tabs & panels" }
                        CardDescription { "Tabbed navigation with content surfaces that stay in sync." }
                    }
                    CardContent {
                        Tabs {
                            default_value: "overview",
                            TabsList {
                                TabsTrigger { value: "overview", "Overview" }
                                TabsTrigger { value: "analytics", "Analytics" }
                                TabsTrigger { value: "reports", "Reports" }
                            }
                            TabsContent {
                                value: "overview",
                                div { class: "ui-stack",
                                    Label { html_for: "overview-search", "Search" }
                                    Input { id: "overview-search", placeholder: "Search docs..." }
                                    SpanHelper { "Triggers share the same focus ring and sizing as the original UI kit." }
                                }
                            }
                            TabsContent {
                                value: "analytics",
                                div { class: "ui-stack",
                                    SpanHelper { "Analytics aggregates live metrics and shows their progress." }
                                    Progress { value: 64.0, max: 100.0 }
                                }
                            }
                            TabsContent {
                                value: "reports",
                                div { class: "ui-stack",
                                    SpanHelper { "Generate PDF, CSV, or scheduled exports directly from here." }
                                    Button { variant: ButtonVariant::Secondary, "Create report" }
                                }
                            }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Dialogs & overlays" }
                        CardDescription { "Popover, hover card, dialogs, sheet, and toast examples." }
                    }
                    CardContent {
                        div { class: "ui-cluster",
                            Button {
                                variant: ButtonVariant::Secondary,
                                on_click: move |_| {
                                    let mut signal = dialog_signal.clone();
                                    signal.set(true);
                                },
                                "Open dialog"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                on_click: move |_| {
                                    let mut signal = sheet_signal.clone();
                                    signal.set(true);
                                },
                                "Open sheet"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                on_click: move |_| {
                                    let mut signal = toast_signal.clone();
                                    signal.set(true);
                                },
                                "Notify me"
                            }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Popover" }
                            Popover {
                                placement: "bottom".to_string(),
                                trigger: rsx! { Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Toggle popover" } },
                                content: rsx! { SpanHelper { "Choose the dialog or sheet you want to configure." } },
                            }
                        }
                        div { class: "ui-stack",
                            SpanHelper { "Hover card" }
                            HoverCard {
                                trigger: rsx! { Badge { variant: BadgeVariant::Secondary, "Hover me" } },
                                content: rsx! { span { style: "font-size: 0.8rem; color: hsl(var(--muted-foreground));", "Preview contextual information instantly." } },
                            }
                        }
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Alerts & extras" }
                        CardDescription { "Feedback surfaces, accordions, and avatar fallbacks." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            Alert {
                                title: Some("Heads up!".to_string()),
                                "We just shipped async server functions to production."
                            }
                            Alert {
                                variant: AlertVariant::Destructive,
                                title: Some("Deployment failed".to_string()),
                                "Check the build logs and retry once the issue is resolved."
                            }
                        }
                        Separator { style: "margin: 1rem 0;" }
                        Accordion {
                            collapsible: true,
                            default_value: Some("item-1".to_string()),
                            AccordionItem {
                                value: "item-1".to_string(),
                                AccordionTrigger { "What is shadcn/ui?" }
                                AccordionContent {
                                    "A collection of unstyled, accessible primitives built on top of Radix, ready for your design system."
                                }
                            }
                            AccordionItem {
                                value: "item-2".to_string(),
                                AccordionTrigger { "Does this work with Dioxus?" }
                                AccordionContent {
                                    "Yes! These components mirror the shadcn/ui ergonomics using Dioxus 0.7 signals."
                                }
                            }
                        }
                        Separator { style: "margin: 1rem 0;" }
                        div { class: "ui-cluster",
                            Tooltip {
                                label: "Ada Lovelace",
                                Avatar {
                                    alt: Some("Ada Lovelace".to_string()),
                                    fallback: Some("AL".to_string()),
                                }
                            }
                            Avatar {
                                alt: Some("Grace Hopper".to_string()),
                                fallback: Some("GH".to_string()),
                            }
                        }
                    }
                }
            }
            Dialog {
                open: dialog_signal.clone(),
                title: Some("Create project".to_string()),
                description: Some("Configure the new analytics workspace.".to_string()),
                div { class: "ui-stack",
                    Label { html_for: "dialog-name", "Project name" }
                    Input { id: "dialog-name", placeholder: "Analytics redesign" }
                }
            }
            Sheet {
                open: sheet_signal.clone(),
                side: SheetSide::Right,
                title: Some("Activity log".to_string()),
                description: Some("Review the latest changes from your teammates.".to_string()),
                div {
                    class: "ui-stack",
                    SpanHelper { "Today" }
                    ul {
                        style: "display: flex; flex-direction: column; gap: 0.5rem; font-size: 0.85rem;",
                        li { "Maria added new metrics to the dashboard." }
                        li { "Evan approved the Q2 launch plan." }
                        li { "Ada commented on revenue projections." }
                    }
                }
            }
            ToastViewport {
                Toast {
                    open: toast_open(),
                    title: Some("Changes saved".to_string()),
                    description: Some("We synced your workspace preferences.".to_string()),
                    on_close: move |_| {
                        let mut signal = toast_signal.clone();
                        signal.set(false);
                    },
                }
            }
        }
    }
}

#[component]
fn SpanHelper(children: Element) -> Element {
    rsx! { span { class: "ui-field-helper", {children} } }
}
