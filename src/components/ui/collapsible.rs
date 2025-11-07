use dioxus::prelude::*;
use super::utils::merge_class;
#[derive(Clone)]
struct CollapsibleContext {
    open: Signal<bool>,
    on_change: Option<EventHandler<bool>>,
}

#[component]
pub fn Collapsible(
    mut open: Signal<bool>,
    #[props(optional)] on_open_change: Option<EventHandler<bool>>,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-collapsible", class);
    let context = CollapsibleContext {
        open: open.clone(),
        on_change: on_open_change.clone(),
    };

    use_context_provider(|| context);

    rsx! {
        div {
            class: classes,
            "data-state": if open() { "open" } else { "closed" },
            {children}
        }
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[props(into, default)] class: Option<String>,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    let context = use_context::<CollapsibleContext>();
    let classes = merge_class("ui-collapsible-trigger", class);
    let mut open_signal = context.open.clone();
    let on_change = context.on_change.clone();
    let is_open = open_signal();

    rsx! {
        button {
            class: classes,
            disabled,
            "data-state": if is_open { "open" } else { "closed" },
            onclick: move |_| {
                if disabled {
                    return;
                }
                let next = !open_signal();
                open_signal.set(next);
                if let Some(handler) = on_change.clone() {
                    handler.call(next);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn CollapsibleContent(
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let context = use_context::<CollapsibleContext>();
    let classes = merge_class("ui-collapsible-content", class);
    let is_open = (context.open)();

    rsx! {
        div {
            class: classes,
            "data-state": if is_open { "open" } else { "closed" },
            if is_open {
                {children}
            }
        }
    }
}
