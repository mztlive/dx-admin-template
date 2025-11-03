use dioxus::prelude::*;

#[component]
pub fn HoverCard(trigger: Element, content: Element) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        span {
            style: "position: relative; display: inline-flex;",
            onmouseenter: move |_| open.set(true),
            onmouseleave: move |_| open.set(false),
            {trigger}
            if open() {
                div {
                    class: "ui-hovercard",
                    style: "left: 50%; bottom: 100%; transform: translate(-50%, -0.75rem);",
                    {content}
                }
            }
        }
    }
}
