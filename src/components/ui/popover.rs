use dioxus::prelude::*;

#[component]
pub fn Popover(
    trigger: Element,
    content: Element,
    #[props(into, default = "bottom".to_string())] placement: String,
) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        span {
            style: "position: relative; display: inline-flex;",
            onclick: move |_| open.set(!open()),
            tabindex: 0,
            onfocusout: move |_| open.set(false),
            {trigger}
            if open() {
                div {
                    class: "ui-popover",
                    "data-placement": placement.clone(),
                    style: match placement.as_str() {
                        "top" => "left: 50%; bottom: 100%;",
                        "bottom" => "left: 50%; top: 100%;",
                        "left" => "right: 100%; top: 50%; transform: translate(-0.75rem, -50%);",
                        "right" => "left: 100%; top: 50%; transform: translate(0.75rem, -50%);",
                        _ => "left: 50%; top: 100%;"
                    },
                    onclick: move |event| event.stop_propagation(),
                    {content}
                }
            }
        }
    }
}
