use super::utils::merge_class;
use crate::components::ui::Label;
use dioxus::prelude::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormMessageVariant {
    Helper,
    Error,
}

impl FormMessageVariant {
    fn as_str(&self) -> &'static str {
        match self {
            FormMessageVariant::Helper => "helper",
            FormMessageVariant::Error => "error",
        }
    }
}

impl Default for FormMessageVariant {
    fn default() -> Self {
        FormMessageVariant::Helper
    }
}

#[component]
pub fn FormField(
    #[props(into, default)] class: Option<String>,
    #[props(optional)] label: Option<String>,
    #[props(optional)] helper_text: Option<String>,
    #[props(optional)] description: Option<String>,
    #[props(optional)] error: Option<Signal<Option<String>>>,
    #[props(into, default)] id: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-form-field", class);
    let id_attr = id.unwrap_or_default();
    let error_signal = error;
    let current_error = error_signal.map(|signal| signal()).flatten();

    rsx! {
        div {
            class: classes,
            if let Some(label_text) = label {
                Label {
                    html_for: id_attr.clone(),
                    "{label_text}"
                }
            }
            if let Some(description_text) = description {
                FormMessage {
                    variant: FormMessageVariant::Helper,
                    class: Some("ui-field-helper".to_string()),
                    "{description_text}"
                }
            }
            {children}
            if let Some(helper) = helper_text {
                FormMessage {
                    variant: FormMessageVariant::Helper,
                    class: Some("ui-field-helper".to_string()),
                    "{helper}"
                }
            }
            if let Some(message) = current_error {
                FormMessage {
                    variant: FormMessageVariant::Error,
                    "{message}"
                }
            }
        }
    }
}

#[component]
pub fn FormMessage(
    #[props(default)] variant: FormMessageVariant,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let classes = merge_class("ui-form-message", class);

    rsx! {
        div {
            class: classes,
            "data-variant": variant.as_str(),
            {children}
        }
    }
}
