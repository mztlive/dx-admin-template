use dioxus::prelude::*;

#[component]
pub fn Tooltip(
    #[props(into)] label: String,
    #[props(default = 0)]
    #[allow(unused)]
    delay_ms: u64,
    children: Element,
) -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        span {
            class: "ui-tooltip-wrapper",
            tabindex: 0,
            onmouseenter: move |_| visible.set(true),
            onmouseleave: move |_| visible.set(false),
            onfocusin: move |_| visible.set(true),
            onfocusout: move |_| visible.set(false),
            {children}
            span {
                class: "ui-tooltip-bubble",
                "data-state": if visible() { "visible" } else { "hidden" },
                "{label}",
            }
        }
    }
}
