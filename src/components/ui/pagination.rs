use dioxus::prelude::*;
use super::button::{Button, ButtonVariant, ButtonSize};

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

    let buttons = calculate_page_buttons(total_pages, current());

    rsx! {
        nav {
            class: "ui-pagination",
            aria_label: "Pagination",
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                disabled: current() <= 1,
                on_click: move |_| {
                    let new_page = current().saturating_sub(1).max(1);
                    current.set(new_page);
                    if let Some(cb) = on_page_change.as_ref() {
                        cb.call(new_page);
                    }
                },
                "Prev"
            }
            for page in buttons {
                {
                    if page == 0 {
                        rsx! { 
                            span { 
                                class: "ui-pagination-ellipsis", 
                                "…" 
                            } 
                        }
                    } else {
                        let is_active = current() == page;
                        rsx! {
                            Button {
                                variant: if is_active { ButtonVariant::Default } else { ButtonVariant::Ghost },
                                size: ButtonSize::Sm,
                                on_click: move |_| {
                                    let new_page = page.max(1).min(total_pages.max(1));
                                    current.set(new_page);
                                    if let Some(cb) = on_page_change.as_ref() {
                                        cb.call(new_page);
                                    }
                                },
                                "{page}"
                            }
                        }
                    }
                }
            }
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                disabled: current() >= total_pages.max(1),
                on_click: move |_| {
                    let new_page = (current() + 1).min(total_pages.max(1));
                    current.set(new_page);
                    if let Some(cb) = on_page_change.as_ref() {
                        cb.call(new_page);
                    }
                },
                "Next"
            }
        }
    }
}

fn calculate_page_buttons(total_pages: usize, active: usize) -> Vec<usize> {
    let mut buttons = vec![];
    if total_pages <= 7 {
        buttons.extend(1..=total_pages);
    } else {
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
    buttons
}
