use dioxus::prelude::*;

/// Visual style variants that match shadcn button presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
    Icon,
}

impl ButtonVariant {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "default",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Destructive => "destructive",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Link => "link",
            ButtonVariant::Icon => "icon",
        }
    }
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Default
    }
}

/// Sizing presets lifted from the shadcn button component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl ButtonSize {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Default => "default",
            ButtonSize::Sm => "sm",
            ButtonSize::Lg => "lg",
            ButtonSize::Icon => "icon",
        }
    }
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Default
    }
}

/// A faithful port of `Button` from shadcn/ui. Styling is provided by `shadcn.css`.
#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(into, default)] class: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] loading: bool,
    #[props(default = "button".to_string())]
    #[props(into)]
    r#type: String,
    #[props(optional)] on_click: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut classes = String::from("ui-button");
    if let Some(extra) = class.filter(|extra| !extra.trim().is_empty()) {
        classes.push(' ');
        classes.push_str(extra.trim());
    }

    let click_handler = on_click.clone();
    let is_disabled = disabled || loading;

    rsx! {
        button {
            class: classes,
            disabled: is_disabled,
            r#type: r#type,
            "data-variant": variant.as_str(),
            "data-size": size.as_str(),
            "data-loading": if loading { "true" } else { "false" },
            "aria-busy": if loading { "true" } else { "false" },
            onclick: move |event| {
                if loading {
                    event.stop_propagation();
                    return;
                }
                if let Some(handler) = click_handler.clone() {
                    handler.call(event);
                }
            },
            if loading {
                span {
                    class: "ui-button-spinner",
                    "aria-hidden": "true",
                }
            }
            span {
                class: "ui-button-content",
                {children}
            }
        }
    }
}
