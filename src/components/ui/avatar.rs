use dioxus::prelude::*;

#[component]
pub fn Avatar(
    #[props(into, default)] src: Option<String>,
    #[props(into, default)] alt: Option<String>,
    #[props(into, default)] fallback: Option<String>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let initial_missing = src.is_none();
    let mut show_fallback = use_signal(move || initial_missing);

    let class_name = format!(
        "{}{}",
        "ui-avatar",
        class
            .filter(|c| !c.trim().is_empty())
            .map(|c| format!(" {c}"))
            .unwrap_or_default()
    );

    let fallback_text = fallback
        .clone()
        .or_else(|| {
            alt.clone().map(|text| {
                text.split_whitespace()
                    .filter_map(|part| part.chars().next())
                    .take(2)
                    .collect::<String>()
            })
        })
        .unwrap_or_else(|| "??".to_string())
        .to_uppercase();

    rsx! {
        div {
            class: class_name,
            if let Some(src) = src {
                img {
                    src: src,
                    alt: alt.clone().unwrap_or_default(),
                    onerror: move |_| show_fallback.set(true),
                    onload: move |_| show_fallback.set(false),
                }
            }
            if show_fallback() {
                div { class: "ui-avatar-fallback", "{fallback_text}" }
            }
        }
    }
}
