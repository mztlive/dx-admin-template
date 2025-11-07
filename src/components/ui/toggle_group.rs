use crate::components::ui::Toggle;
use dioxus::prelude::*;
use super::utils::merge_class;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleGroupMode {
    Single,
    Multiple,
}

impl Default for ToggleGroupMode {
    fn default() -> Self {
        ToggleGroupMode::Single
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleGroupOrientation {
    Horizontal,
    Vertical,
}

impl ToggleGroupOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            ToggleGroupOrientation::Horizontal => "horizontal",
            ToggleGroupOrientation::Vertical => "vertical",
        }
    }
}

impl Default for ToggleGroupOrientation {
    fn default() -> Self {
        ToggleGroupOrientation::Horizontal
    }
}

#[derive(Clone)]
struct ToggleGroupContext {
    values: Signal<Vec<String>>,
    mode: ToggleGroupMode,
    disabled: bool,
    on_change: Option<EventHandler<Vec<String>>>,
}

#[component]
pub fn ToggleGroup(
    mut values: Signal<Vec<String>>,
    #[props(default)] mode: ToggleGroupMode,
    #[props(default)] orientation: ToggleGroupOrientation,
    #[props(default)] disabled: bool,
    #[props(optional)] on_value_change: Option<EventHandler<Vec<String>>>,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-toggle-group", class);

    let context = ToggleGroupContext {
        values: values.clone(),
        mode,
        disabled,
        on_change: on_value_change.clone(),
    };

    use_context_provider(|| context);

    rsx! {
        div {
            class: classes,
            "data-orientation": orientation.as_str(),
            "data-mode": match mode {
                ToggleGroupMode::Single => "single",
                ToggleGroupMode::Multiple => "multiple",
            },
            "data-disabled": disabled,
            {children}
        }
    }
}

#[component]
pub fn ToggleGroupItem(
    #[props(into)] value: String,
    #[props(into, default)] class: Option<String>,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    let context = use_context::<ToggleGroupContext>();
    let classes = merge_class("ui-toggle-group-item", class);
    let values_signal = context.values.clone();
    let mode = context.mode;
    let mut_disabled = context.disabled || disabled;
    let on_change = context.on_change.clone();
    let is_active = values_signal().contains(&value);

    rsx! {
        Toggle {
            class: Some(classes),
            pressed: is_active,
            disabled: mut_disabled,
            on_pressed_change: {
                let value = value.clone();
                let mut values_signal = values_signal.clone();
                let on_change = on_change.clone();
                move |next| {
                    values_signal.with_mut(|items| match mode {
                        ToggleGroupMode::Single => {
                            items.clear();
                            if next {
                                items.push(value.clone());
                            }
                        }
                        ToggleGroupMode::Multiple => {
                            if next {
                                if !items.contains(&value) {
                                    items.push(value.clone());
                                }
                            } else if let Some(index) = items.iter().position(|item| item == &value) {
                                items.remove(index);
                            }
                        }
                    });

                    if let Some(handler) = on_change.clone() {
                        handler.call(values_signal());
                    }
                }
            },
            {children}
        }
    }
}
