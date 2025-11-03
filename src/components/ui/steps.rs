use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StepItem {
    pub title: String,
    pub description: Option<String>,
}

impl StepItem {
    pub fn new(title: impl Into<String>, description: Option<impl Into<String>>) -> Self {
        Self {
            title: title.into(),
            description: description.map(|d| d.into()),
        }
    }
}

#[component]
pub fn Steps(#[props(into)] steps: Vec<StepItem>, #[props(default = 1)] current: usize) -> Element {
    let rendered_steps: Vec<_> = steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            let position = index + 1;
            let state = if position < current {
                "complete"
            } else if position == current {
                "active"
            } else {
                "upcoming"
            };
            let indicator_text = if position < current {
                "✓".to_string()
            } else {
                position.to_string()
            };

            rsx! {
                div {
                    class: "ui-step",
                    "data-state": state,
                    span { class: "ui-step-indicator", "{indicator_text}" }
                    span { "{step.title}" }
                    if let Some(description) = &step.description {
                        span { style: "font-size: 0.72rem; color: hsl(var(--muted-foreground));", "{description}" }
                    }
                }
            }
        })
        .collect();

    let step_nodes = rendered_steps;

    rsx! { div { class: "ui-steps", {step_nodes.into_iter()} } }
}
