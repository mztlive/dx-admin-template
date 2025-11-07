use super::button::{Button, ButtonSize, ButtonVariant};
use chrono::{Datelike, Duration, NaiveDate};
use dioxus::prelude::*;
use super::utils::merge_class;#[cfg(not(target_arch = "wasm32"))]
use std::time::SystemTime;
use crate::components::ui::PopoverHandle;

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

fn last_day_of_month(date: NaiveDate) -> NaiveDate {
    let first = first_day_of_month(date);
    let next_month = add_months(first, 1);
    next_month - Duration::days(1)
}

fn quick_range_week(today: NaiveDate) -> DateRange {
    let start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let end = start + Duration::days(6);
    DateRange::new(start, end)
}

fn quick_range_month(today: NaiveDate) -> DateRange {
    let start = first_day_of_month(today);
    let end = last_day_of_month(today);
    DateRange::new(start, end)
}

#[cfg(target_arch = "wasm32")]
fn today_date() -> NaiveDate {
    use js_sys::Date;

    let date = Date::new_0();
    let year = date.get_full_year() as i32;
    let month = (date.get_month() + 1) as u32;
    let day = date.get_date() as u32;
    NaiveDate::from_ymd_opt(year, month, day)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).expect("unix epoch"))
}

#[cfg(not(target_arch = "wasm32"))]
fn today_date() -> NaiveDate {
    let now = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = now.into();
    datetime.date_naive()
}

fn describe_range(range: DateRange) -> String {
    if range.start == range.end {
        range.start.format("%b %d %Y").to_string()
    } else {
        let start_label = range.start.format("%b %d");
        let end_label = range.end.format("%b %d %Y");
        format!("{start_label} → {end_label}")
    }
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
    let today = today_date();

    let initial_month = value()
        .map(|range| range.start)
        .or(initial_month)
        .map(first_day_of_month)
        .unwrap_or_else(|| first_day_of_month(today));
    let month = use_signal(move || initial_month);
    let hover_date = use_signal(|| None::<NaiveDate>);
    let draft_range = use_signal({
        let initial_value = value();
        move || initial_value
    });
    let popover_handle = try_use_context::<PopoverHandle>();
    let classes = merge_class("ui-date-range", class);
    let on_change_handler = on_change.clone();

    {
        let mut draft_signal = draft_range.clone();
        let value_signal = value.clone();
        use_effect(move || {
            draft_signal.set(value_signal());
        });
    }

    let confirmed_range = value();
    let pending_range = draft_range();
    let base_month = month();

    let month_label = |date: NaiveDate| date.format("%B %Y").to_string();

    let preview_primary_text = match (pending_range, confirmed_range) {
        (Some(pending), Some(confirmed)) if pending == confirmed => {
            format!("当前: {}", describe_range(pending))
        }
        (Some(pending), _) => format!("待确认: {}", describe_range(pending)),
        (None, Some(_)) => "待确认: 清除日期".to_string(),
        (None, None) => "选择一个开始日期以创建范围。".to_string(),
    };
    let preview_secondary_text = match (pending_range, confirmed_range) {
        (Some(pending), Some(confirmed)) if pending != confirmed => {
            Some(format!("当前: {}", describe_range(confirmed)))
        }
        (None, Some(confirmed)) => Some(format!("当前: {}", describe_range(confirmed))),
        _ => None,
    };
    let confirm_disabled = match (pending_range, confirmed_range) {
        (None, None) => true,
        (Some(pending), Some(confirmed)) if pending == confirmed => true,
        _ => false,
    };

    rsx! {
        div {
            class: classes,
            div {
                class: "ui-date-range-toolbar",
                div {
                    class: "ui-date-range-nav-group",
                    Button {
                        class: "ui-date-range-nav",
                        variant: ButtonVariant::Ghost,
                        on_click: {
                            let mut month_signal = month.clone();
                            move |_| {
                                let next = add_months(month_signal(), -12);
                                month_signal.set(next);
                            }
                        },
                        "«"
                    }
                    Button {
                        class: "ui-date-range-nav",
                        variant: ButtonVariant::Ghost,
                        on_click: {
                            let mut month_signal = month.clone();
                            move |_| {
                                let next = add_months(month_signal(), -1);
                                month_signal.set(next);
                            }
                        },
                        "‹"
                    }
                }
                div {
                    class: "ui-date-range-labels",
                    span { class: "ui-date-range-title", "{month_label(base_month)}" }
                    span { class: "ui-date-range-title", "{month_label(add_months(base_month, 1))}" }
                }
                div {
                    class: "ui-date-range-nav-group",
                    Button {
                        class: "ui-date-range-nav",
                        variant: ButtonVariant::Ghost,
                        on_click: {
                            let mut month_signal = month.clone();
                            move |_| {
                                let next = add_months(month_signal(), 1);
                                month_signal.set(next);
                            }
                        },
                        "›"
                    }
                    Button {
                        class: "ui-date-range-nav",
                        variant: ButtonVariant::Ghost,
                        on_click: {
                            let mut month_signal = month.clone();
                            move |_| {
                                let next = add_months(month_signal(), 12);
                                month_signal.set(next);
                            }
                        },
                        "»"
                    }
                }
            }
            div {
                class: "ui-date-range-shortcuts",
                Button {
                    class: "ui-date-range-shortcut",
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Secondary,
                    on_click: {
                        let mut draft_signal = draft_range.clone();
                        let mut month_signal = month.clone();
                        let mut hover_signal = hover_date.clone();
                        move |_| {
                            let selected = quick_range_week(today);
                            draft_signal.set(Some(selected));
                            month_signal.set(first_day_of_month(selected.start));
                            hover_signal.set(None);
                        }
                    },
                    "本周"
                }
                Button {
                    class: "ui-date-range-shortcut",
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Secondary,
                    on_click: {
                        let mut draft_signal = draft_range.clone();
                        let mut month_signal = month.clone();
                        let mut hover_signal = hover_date.clone();
                        move |_| {
                            let selected = quick_range_month(today);
                            draft_signal.set(Some(selected));
                            month_signal.set(first_day_of_month(selected.start));
                            hover_signal.set(None);
                        }
                    },
                    "本月"
                }
            }
            div {
                class: "ui-date-range-calendars",
                for offset in 0..2 {
                    {
                        let calendar_month = add_months(base_month, offset);
                        let days = days_for_month(calendar_month);
                        let active_month = calendar_month.month();
                        let range_signal = draft_range.clone();
                        let hover_signal = hover_date.clone();
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
                                            let is_selected_start = preview.map(|range| range.start == day).unwrap_or(false);
                                            let is_selected_end = preview.map(|range| range.end == day).unwrap_or(false);
                                            let is_in_range = preview.map(|range| range.contains(&day)).unwrap_or(false);
                                            rsx! {
                                                button {
                                                    class: "ui-calendar-day",
                                                    r#type: "button",
                                                    "data-state": if is_selected_start || is_selected_end { "selected" } else { "idle" },
                                                    "data-in-range": if is_in_range { "true" } else { "false" },
                                                    "data-outside": if in_current_month { "false" } else { "true" },
                                                    onclick: {
                                                        let day_value = day;
                                                        let mut draft_signal = range_signal.clone();
                                                        let mut hover_signal = hover_signal.clone();
                                                        move |_| {
                                                            let current_range = draft_signal();
                                                            let next_range = match current_range {
                                                                Some(current) if current.start != current.end => {
                                                                    Some(DateRange::new(day_value, day_value))
                                                                }
                                                                Some(current) => {
                                                                    if day_value == current.start {
                                                                        Some(DateRange::new(day_value, day_value))
                                                                    } else {
                                                                        Some(DateRange::new(current.start, day_value))
                                                                    }
                                                                }
                                                                None => Some(DateRange::new(day_value, day_value)),
                                                            };
                                                            draft_signal.set(next_range);
                                                            hover_signal.set(None);
                                                        }
                                                    },
                                                    onmouseenter: {
                                                        let day_value = day;
                                                        let draft_signal = range_signal.clone();
                                                        let mut hover_signal = hover_signal.clone();
                                                        move |_| {
                                                            if let Some(range) = draft_signal() {
                                                                if range.start == range.end {
                                                                    hover_signal.set(Some(day_value));
                                                                }
                                                            }
                                                        }
                                                    },
                                                    onmouseleave: {
                                                        let mut hover_signal = hover_signal.clone();
                                                        move |_| hover_signal.set(None)
                                                    },
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
            div {
                class: "ui-date-range-footer",
                div {
                    class: "ui-date-range-preview",
                    span { class: "ui-date-range-preview-primary", "{preview_primary_text}" }
                    for text in preview_secondary_text.iter() {
                        span { class: "ui-date-range-preview-secondary", "{text}" }
                    }
                }
                div {
                    class: "ui-date-range-controls",
                    Button {
                        class: "ui-date-range-clear",
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        on_click: {
                            let mut draft_signal = draft_range.clone();
                            let mut hover_signal = hover_date.clone();
                            move |_| {
                                draft_signal.set(None);
                                hover_signal.set(None);
                            }
                        },
                        "清除"
                    }
                    Button {
                        class: "ui-date-range-confirm",
                        size: ButtonSize::Sm,
                        disabled: confirm_disabled,
                        on_click: {
                            let mut value_signal = value.clone();
                            let mut hover_signal = hover_date.clone();
                            let draft_signal = draft_range.clone();
                            let on_change = on_change_handler.clone();
                            let popover_handle = popover_handle.clone();
                            move |_| {
                                let selection = draft_signal();
                                if selection.is_none() && value_signal().is_none() {
                                    return;
                                }
                                value_signal.set(selection);
                                if let Some(handler) = on_change.clone() {
                                    handler.call(selection);
                                }
                                if let Some(mut handle) = popover_handle {
                                    handle.state.set(false);
                                }
                                hover_signal.set(None);
                            }
                        },
                        "确定"
                    }
                }
            }
        }
    }
}
