use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

#[component]
pub fn Dialog(
    mut open: Signal<bool>,
    #[props(into, default)] title: Option<String>,
    #[props(into, default)] description: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    if !open() {
        return rsx! { Fragment {} };
    }

    let mut overlay_signal = open.clone();
    let overlay_handler = on_close.clone();
    let mut button_signal = open.clone();
    let button_handler = on_close.clone();

    rsx! {
        div {
            class: "ui-overlay-backdrop",
            onclick: move |_| {
                overlay_signal.set(false);
                if let Some(cb) = overlay_handler.clone() {
                    cb.call(());
                }
            },
            div {
                class: "ui-dialog",
                onclick: move |event| event.stop_propagation(),
                if let Some(title) = title.clone() {
                    div {
                        class: "ui-dialog-header",
                        h3 { class: "ui-dialog-title", "{title}" }
                        if let Some(desc) = description.clone() {
                            p { class: "ui-dialog-description", "{desc}" }
                        }
                    }
                }
                {children}
                div {
                    class: "ui-dialog-footer",
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        on_click: move |_| {
                            button_signal.set(false);
                            if let Some(cb) = button_handler.clone() {
                                cb.call(());
                            }
                        },
                        "Close"
                    }
                }
            }
        }
    }
}
