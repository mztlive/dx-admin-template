use dioxus::prelude::*;

#[derive(Clone)]
struct AccordionContext {
    open_value: Signal<Option<String>>,
    collapsible: bool,
}

impl AccordionContext {
    fn is_open(&self, value: &str) -> bool {
        matches!((self.open_value)(), Some(current) if current == value)
    }

    fn toggle(&self, value: String) {
        let mut state = self.open_value.clone();
        if self.is_open(&value) {
            if self.collapsible {
                state.set(None);
            }
        } else {
            state.set(Some(value));
        }
    }
}

#[derive(Clone)]
struct AccordionItemContext {
    value: String,
    root: AccordionContext,
}

#[component]
pub fn Accordion(
    #[props(default)] collapsible: bool,
    #[props(into, default)] default_value: Option<String>,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let initial_value = default_value.clone();
    let state = use_signal(move || initial_value.clone());

    let context = AccordionContext {
        open_value: state,
        collapsible,
    };

    use_context_provider(|| context.clone());

    let class_name = format!(
        "{}{}",
        "ui-accordion",
        class
            .filter(|c| !c.trim().is_empty())
            .map(|c| format!(" {c}"))
            .unwrap_or_default()
    );

    rsx! {
        div {
            class: class_name,
            {children}
        }
    }
}

#[component]
pub fn AccordionItem(
    #[props(into)] value: String,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let root = use_context::<AccordionContext>();
    let item_context = AccordionItemContext {
        value: value.clone(),
        root: root.clone(),
    };
    use_context_provider(|| item_context);

    let class_name = format!(
        "{}{}",
        "ui-accordion-item",
        class
            .filter(|c| !c.trim().is_empty())
            .map(|c| format!(" {c}"))
            .unwrap_or_default()
    );

    rsx! {
        div {
            class: class_name,
            "data-state": if root.is_open(&value) { "open" } else { "closed" },
            {children}
        }
    }
}

#[component]
pub fn AccordionTrigger(children: Element) -> Element {
    let item = use_context::<AccordionItemContext>();
    let is_open = item.root.is_open(&item.value);
    let value = item.value.clone();
    let root = item.root.clone();

    rsx! {
        button {
            class: "ui-accordion-trigger",
            "data-state": if is_open { "open" } else { "closed" },
            onclick: move |_| root.toggle(value.clone()),
            {children}
            span {
                style: "font-size: 0.8rem; opacity: 0.6;",
                if is_open { "−" } else { "+" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(children: Element) -> Element {
    let item = use_context::<AccordionItemContext>();
    let is_open = item.root.is_open(&item.value);

    rsx! {
        div {
            class: "ui-accordion-content",
            "data-state": if is_open { "open" } else { "closed" },
            if is_open {
                {children}
            }
        }
    }
}
