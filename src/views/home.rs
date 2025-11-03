use crate::components::{
    ui::{
        Accordion, AccordionContent, AccordionItem, AccordionTrigger, Alert, AlertVariant, Avatar,
        Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardContent, CardDescription,
        CardFooter, CardHeader, CardTitle, Checkbox, DropdownMenu, DropdownMenuItem, Input, Label,
        Progress, RadioGroup, RadioGroupItem, Select, SelectOption, Separator,
        SeparatorOrientation, Slider, Switch, Tabs, TabsContent, TabsList, TabsTrigger, Textarea,
        Tooltip,
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
                        CardTitle { "Select & menus" }
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
        }
    }
}

#[component]
fn SpanHelper(children: Element) -> Element {
    rsx! { span { class: "ui-field-helper", {children} } }
}
