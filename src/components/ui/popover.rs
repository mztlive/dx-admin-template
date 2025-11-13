use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct PopoverHandle {
    pub state: Signal<bool>,
}

#[component]
pub fn Popover(
    trigger: Element,
    content: Element,
    #[props(into, default = "bottom".to_string())] placement: String,
) -> Element {
    let mut open = use_signal(|| false);
    use_context_provider(|| PopoverHandle {
        state: open.clone(),
    });

    rsx! {
        div {
            class: "ui-popover-trigger",
            style: "position: relative; display: inline-flex; align-items: center;",
            onmousedown: move |_| open.set(!open()),
            {trigger}
            if open() {
                div {
                    style: "position: fixed; inset: 0; z-index: 40; background: transparent; pointer-events: auto;",
                    onmousedown: {
                        let mut open_signal = open.clone();
                        move |event| {
                            event.stop_propagation();
                            open_signal.set(false);
                        }
                    },
                    onwheel: {
                        let mut open_signal = open.clone();
                        move |event| {
                            event.stop_propagation();
                            open_signal.set(false);
                        }
                    },
                    ontouchmove: {
                        let mut open_signal = open.clone();
                        move |event| {
                            event.stop_propagation();
                            open_signal.set(false);
                        }
                    },
                }
                div {
                    class: "ui-popover",
                    "data-placement": placement.clone(),
                    style: match placement.as_str() {
                        "top" => "position: absolute; left: 50%; bottom: 100%; transform: translate(-50%, -0.75rem); z-index: 50;",
                        "bottom" => "position: absolute; left: 50%; top: 100%; transform: translate(-50%, 0.75rem); z-index: 50;",
                        "left" => "position: absolute; right: 100%; top: 50%; transform: translate(-0.75rem, -50%); z-index: 50;",
                        "right" => "position: absolute; left: 100%; top: 50%; transform: translate(0.75rem, -50%); z-index: 50;",
                        _ => "position: absolute; left: 50%; top: 100%; transform: translate(-50%, 0.75rem); z-index: 50;"
                    },
                    onmousedown: move |event| event.stop_propagation(),
                    onclick: move |event| event.stop_propagation(),
                    {content}
                }
            }
        }
    }
}
