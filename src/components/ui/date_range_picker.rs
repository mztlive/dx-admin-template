use chrono::{Datelike, Duration, NaiveDate};
use dioxus::prelude::*;

fn merge_class(base: &str, extra: Option<String>) -> String {
    if let Some(extra) = extra.filter(|extra| !extra.trim().is_empty()) {
        format!("{base} {}", extra.trim())
    } else {
        base.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl DateRange {
    pub fn new(a: NaiveDate, b: NaiveDate) -> Self {
        if a <= b {
            Self { start: a, end: b }
        } else {
            Self { start: b, end: a }
        }
    }

    pub fn contains(&self, date: &NaiveDate) -> bool {
        *date >= self.start && *date <= self.end
    }
}

const WEEKDAY_LABELS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn first_day_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

fn add_months(date: NaiveDate, offset: i32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() as i32 + offset;
    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }
    NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap_or(date)
}

fn days_for_month(month_start: NaiveDate) -> Vec<NaiveDate> {
    let start_offset = month_start.weekday().num_days_from_monday() as i64;
    let mut cursor = month_start - Duration::days(start_offset);
    let mut days = Vec::with_capacity(42);
    for _ in 0..42 {
        days.push(cursor);
        cursor += Duration::days(1);
    }
    days
}

fn range_preview(range: Option<DateRange>, hovered: Option<NaiveDate>) -> Option<DateRange> {
    match (range, hovered) {
        (Some(active), Some(hover)) if active.start == active.end => {
            Some(DateRange::new(active.start, hover))
        }
        (Some(active), _) => Some(active),
        (None, _) => None,
    }
}

#[component]
pub fn DateRangePicker(
    mut value: Signal<Option<DateRange>>,
    #[props(optional)] on_change: Option<EventHandler<Option<DateRange>>>,
    #[props(optional)] initial_month: Option<NaiveDate>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let initial_month = value()
        .map(|range| range.start)
        .or(initial_month)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid default date"));
    let month = use_signal(move || first_day_of_month(initial_month));
    let hover_date = use_signal(|| None::<NaiveDate>);
    let classes = merge_class("ui-date-range", class);
    let on_change_handler = on_change.clone();

    let active_range = value();
    let month_label = |date: NaiveDate| date.format("%B %Y").to_string();

    rsx! {
        div {
            class: classes,
            div {
                class: "ui-date-range-toolbar",
                button {
                    class: "ui-date-range-nav",
                    r#type: "button",
                    "aria-label": "Previous month",
                    onclick: {
                        let mut month_signal = month.clone();
                        move |_| {
                            let next = add_months(month_signal(), -1);
                            month_signal.set(next);
                        }
                    },
                    "‹"
                }
                div {
                    class: "ui-date-range-labels",
                    span { class: "ui-date-range-title", "{month_label(month())}" }
                    span { class: "ui-date-range-title", "{month_label(add_months(month(), 1))}" }
                }
                button {
                    class: "ui-date-range-nav",
                    r#type: "button",
                    "aria-label": "Next month",
                    onclick: {
                        let mut month_signal = month.clone();
                        move |_| {
                            let next = add_months(month_signal(), 1);
                            month_signal.set(next);
                        }
                    },
                    "›"
                }
            }
            div {
                class: "ui-date-range-preview",
                match active_range {
                    Some(range) if range.start != range.end => {
                        let start_label = range.start.format("%b %d").to_string();
                        let end_label = range.end.format("%b %d %Y").to_string();
                        rsx! { span { "{start_label} → {end_label}" } }
                    }
                    Some(range) => {
                        let label = range.start.format("%b %d %Y").to_string();
                        rsx! { span { "Selected {label}" } }
                    }
                    None => rsx! { span { "Pick a start date to begin the range." } },
                }
            }
            div {
                class: "ui-date-range-calendars",
                for offset in 0..2 {
                    {
                        let calendar_month = add_months(month(), offset);
                        let days = days_for_month(calendar_month);
                        let active_month = calendar_month.month();
                        let range_signal = value.clone();
                        let hover_signal = hover_date.clone();
                        let on_change_handler = on_change_handler.clone();

                        rsx! {
                            div {
                                class: "ui-date-range-calendar",
                                div { class: "ui-calendar-weekdays",
                                    for label in WEEKDAY_LABELS {
                                        span { class: "ui-calendar-weekday", "{label}" }
                                    }
                                }
                                div { class: "ui-calendar-grid",
                                    for day in days {
                                        {
                                            let preview = range_preview(range_signal(), hover_signal());
                                            let in_current_month = day.month() == active_month;
                                            let mut range_signal = range_signal.clone();
                                            let mut hover_signal = hover_signal.clone();
                                            let is_selected_start = preview
                                                .map(|range| range.start == day)
                                                .unwrap_or(false);
                                            let is_selected_end = preview
                                                .map(|range| range.end == day)
                                                .unwrap_or(false);
                                            let is_in_range = preview
                                                .map(|range| range.contains(&day))
                                                .unwrap_or(false);

                                            rsx! {
                                                button {
                                                    class: "ui-calendar-day",
                                                    r#type: "button",
                                                    "data-state": if is_selected_start || is_selected_end { "selected" } else { "idle" },
                                                    "data-in-range": if is_in_range { "true" } else { "false" },
                                                    "data-outside": if in_current_month { "false" } else { "true" },
                                                    onclick: {
                                                        let day_value = day;
                                                        let on_change_handler = on_change_handler.clone();
                                                        move |_| {
                                                            let current_range = range_signal();
                                                            match current_range {
                                                                Some(current) if current.start != current.end => {
                                                                    let new_range = DateRange::new(day_value, day_value);
                                                                    range_signal.set(Some(new_range));
                                                                    if let Some(handler) = on_change_handler.clone() {
                                                                        handler.call(Some(new_range));
                                                                    }
                                                                }
                                                                Some(current) => {
                                                                    if day_value == current.start {
                                                                        let new_range = DateRange::new(day_value, day_value);
                                                                        range_signal.set(Some(new_range));
                                                                        if let Some(handler) = on_change_handler.clone() {
                                                                            handler.call(Some(new_range));
                                                                        }
                                                                    } else {
                                                                        let new_range = DateRange::new(current.start, day_value);
                                                                        range_signal.set(Some(new_range));
                                                                        if let Some(handler) = on_change_handler.clone() {
                                                                            handler.call(Some(new_range));
                                                                        }
                                                                    }
                                                                }
                                                        None => {
                                                            let new_range = DateRange::new(day_value, day_value);
                                                            range_signal.set(Some(new_range));
                                                            if let Some(handler) = on_change_handler.clone() {
                                                                handler.call(Some(new_range));
                                                            }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    onmouseenter: move |_| {
                                                        let range = range_signal();
                                                        if let Some(range) = range {
                                                            if range.start == range.end {
                                                                hover_signal.set(Some(day));
                                                            }
                                                        }
                                                    },
                                                    onmouseleave: move |_| hover_signal.set(None),
                                                    "{day.day()}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
