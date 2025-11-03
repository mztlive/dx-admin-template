use dioxus::prelude::*;

#[component]
pub fn ToastViewport(children: Element) -> Element {
    rsx! { div { class: "ui-toast-container", {children} } }
}

#[component]
pub fn Toast(
    #[props(default)] open: bool,
    #[props(into, default)] title: Option<String>,
    #[props(into, default)] description: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
) -> Element {
    if !open {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            class: "ui-toast",
            if let Some(title) = title {
                h4 { style: "font-weight: 600; font-size: 0.95rem;", "{title}" }
            }
            if let Some(desc) = description {
                p { style: "font-size: 0.8rem; color: hsl(var(--muted-foreground));", "{desc}" }
            }
            if on_close.is_some() {
                button {
                    class: "ui-page-button",
                    style: "align-self: flex-end; height: 2rem;",
                    onclick: move |_| {
                        if let Some(cb) = on_close.clone() {
                            cb.call(());
                        }
                    },
                    "Dismiss"
                }
            }
        }
    }
}
