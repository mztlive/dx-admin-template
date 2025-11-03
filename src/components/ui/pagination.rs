use dioxus::prelude::*;

#[component]
pub fn Pagination(
    total_pages: usize,
    current_page: usize,
    #[props(optional)] on_page_change: Option<EventHandler<usize>>,
) -> Element {
    let mut current = use_signal(move || current_page.max(1).min(total_pages.max(1)));
    use_effect(move || {
        if current() != current_page {
            current.set(current_page.max(1).min(total_pages.max(1)));
        }
    });

    let on_change = on_page_change.clone();

    let mut buttons = vec![];
    if total_pages <= 7 {
        buttons.extend(1..=total_pages);
    } else {
        let active = current();
        buttons.extend([1, 2]);
        if active > 4 {
            buttons.push(0); // ellipsis indicator
        }
        let start = active.saturating_sub(1).max(3);
        let end = (active + 1).min(total_pages - 2);
        for page in start..=end {
            buttons.push(page);
        }
        if active + 2 < total_pages - 1 {
            buttons.push(0);
        }
        buttons.extend([total_pages - 1, total_pages]);
    }

    let page_nodes: Vec<_> = buttons
        .iter()
        .map(|page| {
            if *page == 0 {
                rsx! { span { class: "ui-page-button", style: "pointer-events: none;", "…" } }
            } else {
                let mut page_signal = current.clone();
                let page_handler = on_change.clone();
                let target = *page;
                rsx! {
                    button {
                        class: "ui-page-button",
                        "data-active": if page_signal() == target { "true" } else { "false" },
                        onclick: move |_| {
                            let new_page = target.max(1).min(total_pages.max(1));
                            page_signal.set(new_page);
                            if let Some(cb) = page_handler.clone() {
                                cb.call(new_page);
                            }
                        },
                        "{target}"
                    }
                }
            }
        })
        .collect();

    let mut prev_signal = current.clone();
    let prev_handler = on_change.clone();
    let mut next_signal = current.clone();
    let next_handler = on_change.clone();

    rsx! {
        nav {
            class: "ui-pagination",
            aria_label: "Pagination",
            button {
                class: "ui-page-button",
                disabled: prev_signal() <= 1,
                onclick: move |_| {
                    let new_page = prev_signal().saturating_sub(1).max(1);
                    prev_signal.set(new_page);
                    if let Some(cb) = prev_handler.clone() {
                        cb.call(new_page);
                    }
                },
                "Prev"
            }
            {page_nodes.into_iter()}
            button {
                class: "ui-page-button",
                disabled: next_signal() >= total_pages.max(1),
                onclick: move |_| {
                    let new_page = (next_signal() + 1).min(total_pages.max(1));
                    next_signal.set(new_page);
                    if let Some(cb) = next_handler.clone() {
                        cb.call(new_page);
                    }
                },
                "Next"
            }
        }
    }
}
