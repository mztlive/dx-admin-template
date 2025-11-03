use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetSide {
    Left,
    Right,
}

impl SheetSide {
    fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Left => "left",
            SheetSide::Right => "right",
        }
    }
}

impl Default for SheetSide {
    fn default() -> Self {
        SheetSide::Right
    }
}

#[component]
pub fn Sheet(
    mut open: Signal<bool>,
    #[props(default)] side: SheetSide,
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
            class: "ui-sheet-backdrop",
            onclick: move |_| {
                overlay_signal.set(false);
                if let Some(cb) = overlay_handler.clone() {
                    cb.call(());
                }
            },
        }
        div {
            class: "ui-sheet",
            "data-side": side.as_str(),
            onclick: move |event| event.stop_propagation(),
            if let Some(title) = title.clone() {
                h3 { class: "ui-dialog-title", "{title}" }
            }
            if let Some(desc) = description.clone() {
                p { class: "ui-dialog-description", "{desc}" }
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
