use crate::components::{
    ui::{
        Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardContent, CardDescription,
        CardFooter, CardHeader, CardTitle, Checkbox, Input, Label, Progress, RadioGroup,
        RadioGroupItem, Separator, SeparatorOrientation, Slider, Switch, Tabs, TabsContent,
        TabsList, TabsTrigger, Textarea,
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
    let mut accepted_terms = use_signal(|| false);
    let mut email_notifications = use_signal(|| true);
    let mut slider_value = use_signal(|| 42.0f32);
    let mut contact_method = use_signal(|| "email".to_string());
    let mut newsletter_opt_in = use_signal(|| true);
    let mut dark_mode = use_signal(|| false);
    let slider_value_signal = slider_value.clone();
    let contact_method_signal = contact_method.clone();
    let intensity_text = move || format!("Accent intensity: {:.0}%", slider_value_signal());
    let contact_text = move || format!("Preferred contact: {}", contact_method_signal());

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
                                on_value_change: move |val| slider_value.set(val),
                            }
                            Progress { value: slider_value(), max: 100.0 }
                            SpanHelper { "{intensity_text()}" }
                        }
                        div { class: "ui-bleed",
                            div { class: "ui-cluster",
                                Checkbox {
                                    id: Some("accept-terms".to_string()),
                                    checked: accepted_terms(),
                                    on_checked_change: move |state| accepted_terms.set(state),
                                }
                                Label { html_for: "accept-terms", "Agree to terms" }
                            }
                            div { class: "ui-cluster",
                                Label { html_for: "profile-emails", "Email notifications" }
                                Switch {
                                    id: Some("profile-emails".to_string()),
                                    checked: email_notifications(),
                                    on_checked_change: move |state| email_notifications.set(state),
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
                        CardTitle { "Selection controls" }
                        CardDescription { "Checkboxes, switches, and radio groups stay in sync with signals." }
                    }
                    CardContent {
                        div { class: "ui-stack",
                            div { class: "ui-cluster",
                                Checkbox {
                                    id: Some("newsletter-opt".to_string()),
                                    checked: newsletter_opt_in(),
                                    on_checked_change: move |state| newsletter_opt_in.set(state),
                                }
                                Label { html_for: "newsletter-opt", "Subscribe to newsletter" }
                            }
                            div { class: "ui-cluster",
                                Label { html_for: "dark-mode", "Dark mode" }
                                Switch {
                                    id: Some("dark-mode".to_string()),
                                    checked: dark_mode(),
                                    on_checked_change: move |state| dark_mode.set(state),
                                }
                            }
                            Separator { style: "margin: 0.75rem 0;" }
                            RadioGroup {
                                default_value: contact_method(),
                                on_value_change: move |value| contact_method.set(value),
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
            }
        }
    }
}

#[component]
fn SpanHelper(children: Element) -> Element {
    rsx! { span { class: "ui-field-helper", {children} } }
}
