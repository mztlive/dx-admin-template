use crate::time::{Duration, NaiveDate};
use dioxus::prelude::*;
use super::utils::merge_class;
const WEEKDAY_LABELS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn first_day_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

fn next_month(date: NaiveDate) -> NaiveDate {
    let year = if date.month() == 12 {
        date.year() + 1
    } else {
        date.year()
    };
    let month = if date.month() == 12 {
        1
    } else {
        date.month() + 1
    };
    NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(date)
}

fn previous_month(date: NaiveDate) -> NaiveDate {
    let year = if date.month() == 1 {
        date.year() - 1
    } else {
        date.year()
    };
    let month = if date.month() == 1 {
        12
    } else {
        date.month() - 1
    };
    NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(date)
}

#[component]
pub fn Calendar(
    #[props(into)] initial_month: NaiveDate,
    #[props(optional)] selected: Option<NaiveDate>,
    #[props(default)] show_outside_days: bool,
    #[props(optional)] on_select: Option<EventHandler<NaiveDate>>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let starting_month = first_day_of_month(initial_month);
    let month = use_signal(move || starting_month);
    let selection = use_signal(move || selected);
    let mut month_signal = month.clone();
    let on_select_handler = on_select.clone();

    let active_month = month();
    let month_label = active_month.format("%B %Y").to_string();
    let start_weekday = active_month.weekday().num_days_from_monday() as i64;
    let mut first_visible = active_month - Duration::days(start_weekday);
    let mut days = Vec::with_capacity(42);
    for _ in 0..42 {
        days.push(first_visible);
        first_visible = first_visible + Duration::days(1);
    }

    let current_selection = selection();

    rsx! {
        div {
            class: merge_class("ui-calendar", class),
            div {
                class: "ui-calendar-header",
                button {
                    class: "ui-calendar-nav",
                    r#type: "button",
                    "aria-label": "Go to previous month",
                    onclick: move |_| {
                        month_signal.set(previous_month(active_month));
                    },
                    "‹"
                }
                span { class: "ui-calendar-title", "{month_label}" }
                button {
                    class: "ui-calendar-nav",
                    r#type: "button",
                    "aria-label": "Go to next month",
                    onclick: move |_| {
                        month_signal.set(next_month(active_month));
                    },
                    "›"
                }
            }
            div {
                class: "ui-calendar-weekdays",
                for label in WEEKDAY_LABELS {
                    span { class: "ui-calendar-weekday", "{label}" }
                }
            }
            div {
                class: "ui-calendar-grid",
                for day in days {
                    {
                        let is_current_month = day.month() == active_month.month();
                        let is_selected = current_selection
                            .as_ref()
                            .map(|selected| *selected == day)
                            .unwrap_or(false);
                        let is_disabled = !show_outside_days && !is_current_month;
                        let day_display = day.day();
                        let mut selection_signal = selection.clone();
                        let handler = on_select_handler.clone();

                        rsx! {
                            button {
                                class: "ui-calendar-day",
                                r#type: "button",
                                "data-state": if is_selected { "selected" } else { "idle" },
                                "data-outside": if is_current_month { "false" } else { "true" },
                                disabled: is_disabled,
                                onclick: move |_| {
                                    if !is_disabled {
                                        selection_signal.set(Some(day));
                                        if let Some(callback) = handler.clone() {
                                            callback.call(day);
                                        }
                                    }
                                },
                                "{day_display}"
                            }
                        }
                    }
                }
            }
        }
    }
}
