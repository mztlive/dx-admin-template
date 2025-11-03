use dioxus::prelude::*;

#[component]
pub fn Popover(
    trigger: Element,
    content: Element,
    #[props(into, default = "bottom".to_string())] placement: String,
) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            class: "ui-popover-trigger",
            style: "position: relative; display: inline-flex; align-items: center;",
            onmousedown: move |_| open.set(!open()),
            tabindex: 0,
            onfocusout: move |_| open.set(false),
            {trigger}
            if open() {
                div {
                    class: "ui-popover",
                    "data-placement": placement.clone(),
                    style: match placement.as_str() {
                        "top" => "position: absolute; left: 50%; bottom: 100%; transform: translate(-50%, -0.75rem);",
                        "bottom" => "position: absolute; left: 50%; top: 100%; transform: translate(-50%, 0.75rem);",
                        "left" => "position: absolute; right: 100%; top: 50%; transform: translate(-0.75rem, -50%);",
                        "right" => "position: absolute; left: 100%; top: 50%; transform: translate(0.75rem, -50%);",
                        _ => "position: absolute; left: 50%; top: 100%; transform: translate(-50%, 0.75rem);"
                    },
                    onclick: move |event| event.stop_propagation(),
                    {content}
                }
            }
        }
    }
}
