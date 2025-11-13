use super::utils::merge_class;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsOrientation {
    Horizontal,
    #[allow(dead_code)]
    Vertical,
}

impl TabsOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            TabsOrientation::Horizontal => "horizontal",
            TabsOrientation::Vertical => "vertical",
        }
    }
}

impl Default for TabsOrientation {
    fn default() -> Self {
        TabsOrientation::Horizontal
    }
}

#[derive(Clone)]
struct TabsContext {
    value: Signal<String>,
    on_change: Option<EventHandler<String>>,
}

#[component]
pub fn Tabs(
    #[props(into)] default_value: String,
    #[props(default)] orientation: TabsOrientation,
    #[props(into, default)] class: Option<String>,
    #[props(optional)] on_value_change: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    let initial_value = default_value.clone();
    let selected = use_signal(move || initial_value.clone());

    let context = TabsContext {
        value: selected.clone(),
        on_change: on_value_change.clone(),
    };

    use_context_provider(|| context);

    let classes = merge_class("ui-tabs", class);

    rsx! {
        div {
            class: classes,
            "data-orientation": orientation.as_str(),
            {children}
        }
    }
}

#[component]
pub fn TabsList(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-tabs-nav", class);

    rsx! {
        div {
            class: classes,
            role: "tablist",
            {children}
        }
    }
}

#[component]
pub fn TabsTrigger(
    #[props(into)] value: String,
    #[props(into, default)] class: Option<String>,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    let context = use_context::<TabsContext>();
    let classes = merge_class("ui-tabs-trigger", class);
    let mut selected_signal = context.value.clone();
    let is_active = selected_signal() == value;
    let trigger_value = value.clone();
    let trigger_attr_value = trigger_value.clone();
    let on_change = context.on_change.clone();

    rsx! {
        button {
            class: classes,
            role: "tab",
            "data-state": if is_active { "active" } else { "inactive" },
            "aria-selected": is_active,
            "aria-controls": format!("tab-panel-{}", trigger_value),
            value: trigger_attr_value,
            disabled,
            onclick: move |_event| {
                selected_signal.set(trigger_value.clone());
                if let Some(handler) = on_change.clone() {
                    handler.call(trigger_value.clone());
                }
            },
            {children}
        }
    }
}

#[component]
pub fn TabsContent(
    #[props(into)] value: String,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let context = use_context::<TabsContext>();
    let classes = merge_class("ui-tabs-content", class);
    let selected_signal = context.value.clone();
    let is_active = selected_signal() == value;
    let panel_id = format!("tab-panel-{value}");

    rsx! {
        div {
            class: classes,
            role: "tabpanel",
            id: panel_id,
            hidden: !is_active,
            {children}
        }
    }
}
