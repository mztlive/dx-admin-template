use crate::components::ui::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
    Badge, BadgeVariant, Button, ButtonVariant, Card, CardContent,
    CardDescription, CardFooter, CardHeader, CardTitle,
    Progress, Tabs,
    TabsContent, TabsList, TabsTrigger,
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let kpis = [
        ("Monthly Recurring Revenue", "$82.4k", "+4.1% vs last month"),
        ("Active Accounts", "18,245", "+320 this week"),
        ("Trial Conversion Rate", "32%", "Goal: 40%"),
        ("Support CSAT", "94%", "230 responses"),
    ];
    let experiments = [
        (
            "Guided onboarding checklist",
            "QA",
            "Validating instrumentation with the analytics team.",
        ),
        (
            "Usage-based alerts",
            "Design",
            "Final polish on notification copy before launch.",
        ),
        (
            "Unified billing dashboard",
            "Build",
            "Integration tests are running against staging.",
        ),
    ];
    let adoption_segments = [
        (
            "Onboarding completion",
            74.0f32,
            "Teams finished the setup checklist with no blockers.",
        ),
        (
            "Automation adoption",
            58.0f32,
            "Workflows triggered in the past 7 days across customers.",
        ),
        (
            "Weekly active accounts",
            83.0f32,
            "Organizations engaging with analytics at least twice.",
        ),
    ];
    let alert_feed = [
        (
            "Incident response playbook",
            "Workspace incidents resolved in under 4h for 92% of cases.",
        ),
        (
            "Billing sync delays",
            "Elevated Stripe webhook latency observed over the weekend.",
        ),
        (
            "API deprecations",
            "Notify integrators about the upcoming reporting v2 changes.",
        ),
    ];
    let standup_sections: [(&str, [&str; 2]); 3] = [
        (
            "Platform",
            [
                "Ship metering hotfix across clusters",
                "Pair with data engineering for realtime audits",
            ],
        ),
        (
            "Product",
            [
                "Enable proactive insights beta for 10 design partners",
                "Finalize empty state content review with brand",
            ],
        ),
        (
            "Customer Success",
            [
                "Schedule QBR with Globex expansion team",
                "Collect testimonials for the website refresh",
            ],
        ),
    ];
    let priority_backlog = [
        (
            "Lifecycle emails",
            "Growth",
            "Net-new education journey that nurtures power users.",
        ),
        (
            "Data residency",
            "Platform",
            "Coordinate EU workspace pilot timeline with legal.",
        ),
        (
            "Journeys analytics",
            "Product",
            "Scope dashboards that highlight activation funnels.",
        ),
    ];

    rsx! {
        div {
            class: "dashboard-root",
            div {
                class: "page-heading",
                h1 { "Dashboard overview" }
                p { "Track product health, monitor key funnels, and coordinate the team from one place." }
            }
            section {
                class: "dashboard-kpis",
                style: "display: grid; gap: 16px; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));",
                for (title, value, hint) in kpis {
                    Card {
                        CardHeader {
                            CardTitle { "{title}" }
                            CardDescription { "{hint}" }
                        }
                        CardContent {
                            span {
                                style: "font-size: 2rem; font-weight: 600;",
                                "{value}"
                            }
                        }
                    }
                }
            }
            section {
                class: "dashboard-panels",
                style: "display: grid; gap: 24px; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); align-items: start;",
                Card {
                    CardHeader {
                        CardTitle { "Product health" }
                        CardDescription { "Experiments, adoption, and alerts surfaced from telemetry." }
                    }
                    CardContent {
                        Tabs {
                            default_value: "experiments".to_string(),
                            TabsList {
                                TabsTrigger { value: "experiments".to_string(), "Experiments" }
                                TabsTrigger { value: "adoption".to_string(), "Adoption" }
                                TabsTrigger { value: "alerts".to_string(), "Alerts" }
                            }
                            TabsContent { value: "experiments".to_string(),
                                div {
                                    style: "display: flex; flex-direction: column; gap: 16px;",
                                    for (title, stage, note) in experiments {
                                        div {
                                            style: "display: flex; flex-direction: column; gap: 6px; background: hsl(var(--muted)); padding: 16px; border-radius: calc(var(--radius) - 2px);",
                                            div {
                                                style: "display: flex; justify-content: space-between; align-items: center; gap: 12px;",
                                                span { style: "font-weight: 600;", "{title}" }
                                                Badge { variant: BadgeVariant::Secondary, "{stage}" }
                                            }
                                            p { style: "color: hsl(var(--muted-foreground)); margin: 0;", "{note}" }
                                        }
                                    }
                                }
                            }
                            TabsContent { value: "adoption".to_string(),
                                div {
                                    style: "display: flex; flex-direction: column; gap: 16px;",
                                    for (label, percent, note) in adoption_segments {
                                        div {
                                            style: "display: flex; flex-direction: column; gap: 8px;",
                                            div {
                                                style: "display: flex; justify-content: space-between; align-items: baseline;",
                                                span { style: "font-weight: 600;", "{label}" }
                                                span { style: "font-size: 0.9rem; color: hsl(var(--muted-foreground));", "{percent:.0}%" }
                                            }
                                            Progress { value: percent }
                                            p { style: "color: hsl(var(--muted-foreground)); margin: 0;", "{note}" }
                                        }
                                    }
                                }
                            }
                            TabsContent { value: "alerts".to_string(),
                                div {
                                    style: "display: flex; flex-direction: column; gap: 12px;",
                                    for (title, note) in alert_feed {
                                        div {
                                            style: "display: flex; flex-direction: column; gap: 4px;",
                                            span { style: "font-weight: 600;", "{title}" }
                                            p { style: "color: hsl(var(--muted-foreground)); margin: 0;", "{note}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    CardFooter {
                        Button {
                            variant: ButtonVariant::Secondary,
                            r#type: "button".to_string(),
                            "Open product review"
                        }
                    }
                }
                Card {
                    CardHeader {
                        CardTitle { "Team stand-up" }
                        CardDescription { "Snapshots ready to paste into the leadership sync." }
                    }
                    CardContent {
                        Accordion {
                            default_value: Some("Platform".to_string()),
                            collapsible: true,
                            for (team, updates) in standup_sections {
                                AccordionItem {
                                    value: team.to_string(),
                                    AccordionTrigger { "{team}" }
                                    AccordionContent {
                                        ul {
                                            style: "margin: 0; padding-left: 20px; display: flex; flex-direction: column; gap: 6px;",
                                            for update in updates {
                                                li { "{update}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Card {
                    CardHeader {
                        CardTitle { "Escalation queue" }
                        CardDescription { "Cross-functional work that needs executive unblockers." }
                    }
                    CardContent {
                        div {
                            style: "display: flex; flex-direction: column; gap: 16px;",
                            for (title, owner, note) in priority_backlog {
                                div {
                                    style: "display: flex; flex-direction: column; gap: 4px;",
                                    div {
                                        style: "display: flex; justify-content: space-between; align-items: center;",
                                        span { style: "font-weight: 600;", "{title}" }
                                        Badge { variant: BadgeVariant::Outline, "{owner}" }
                                    }
                                    p { style: "color: hsl(var(--muted-foreground)); margin: 0;", "{note}" }
                                }
                            }
                        }
                    }
                    CardFooter {
                        Button {
                            variant: ButtonVariant::Ghost,
                            r#type: "button".to_string(),
                            "View roadmap"
                        }
                    }
                }
            }
        }
    }
}
