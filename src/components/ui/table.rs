use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::{utils::merge_class, Button, ButtonSize, ButtonVariant};
use crate::components::ui::Checkbox;
use dioxus::prelude::*;
#[component]
pub fn Table(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table", class);

    rsx! {
        table {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableHeader(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-header", class);

    rsx! {
        thead {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableBody(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-body", class);

    rsx! {
        tbody {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableFooter(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-footer", class);

    rsx! {
        tfoot {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableRow(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-row", class);

    rsx! {
        tr {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableHead(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-head", class);

    rsx! {
        th {
            class: classes,
            scope: "col",
            {children}
        }
    }
}

#[component]
pub fn TableCell(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-cell", class);

    rsx! {
        td {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn TableCaption(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let classes = merge_class("ui-table-caption", class);

    rsx! {
        caption {
            class: classes,
            {children}
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TableColumnConfig {
    pub id: String,
    pub label: String,
    pub toggleable: bool,
    pub visible_by_default: bool,
}

#[allow(dead_code)]
impl TableColumnConfig {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            toggleable: true,
            visible_by_default: true,
        }
    }

    pub fn fixed(mut self) -> Self {
        self.toggleable = false;
        self
    }

    pub fn hide_by_default(mut self) -> Self {
        self.visible_by_default = false;
        self
    }
}

#[derive(Clone, PartialEq)]
pub struct TableRowData {
    pub id: String,
    pub cells: HashMap<String, String>,
}

#[allow(dead_code)]
impl TableRowData {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            cells: HashMap::new(),
        }
    }

    pub fn with_cell(mut self, column_id: impl Into<String>, value: impl Into<String>) -> Self {
        self.cells.insert(column_id.into(), value.into());
        self
    }

    pub fn from_pairs<T, K, V>(id: impl Into<String>, cells: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut row = Self::new(id);
        for (column, value) in cells {
            row.cells.insert(column.into(), value.into());
        }
        row
    }
}

#[component]
pub fn InteractiveTable(
    #[props(into)] columns: Vec<TableColumnConfig>,
    #[props(into)] rows: Vec<TableRowData>,
    #[props(into, default)] class: Option<String>,
    #[props(into, default)] table_class: Option<String>,
    #[props(into, default)] default_selected: Option<Vec<String>>,
    #[props(into, default)] empty_state: Option<String>,
    #[props(optional)] on_selection_change: Option<EventHandler<Vec<String>>>,
    #[props(optional)] on_visibility_change: Option<EventHandler<Vec<String>>>,
) -> Element {
    let wrapper_class = merge_class("ui-data-table", class);
    let inner_table_class = merge_class("ui-table", table_class);

    let initial_selected: HashSet<String> =
        default_selected.unwrap_or_default().into_iter().collect();
    let selected_rows = use_signal({
        let initial_selected = initial_selected.clone();
        move || initial_selected.clone()
    });

    let initial_visible: HashSet<String> = {
        let mut visible = HashSet::new();
        for column in &columns {
            if column.visible_by_default || !column.toggleable {
                visible.insert(column.id.clone());
            }
        }
        if visible.is_empty() {
            if let Some(first) = columns.first() {
                visible.insert(first.id.clone());
            }
        }
        visible
    };
    let visible_columns = use_signal({
        let initial_visible = initial_visible.clone();
        move || initial_visible.clone()
    });

    let toggleable_columns = Rc::new(
        columns
            .iter()
            .filter(|column| column.toggleable)
            .cloned()
            .collect::<Vec<_>>(),
    );
    let column_order = Rc::new(
        columns
            .iter()
            .map(|column| column.id.clone())
            .collect::<Vec<_>>(),
    );
    let row_order = Rc::new(rows.iter().map(|row| row.id.clone()).collect::<Vec<_>>());
    let min_visible_columns = max(
        columns.iter().filter(|column| !column.toggleable).count(),
        1,
    );

    let selection_handler = on_selection_change.clone();
    let visibility_handler = on_visibility_change.clone();
    let columns_menu_open = use_signal(|| false);

    let selected_snapshot = selected_rows();
    let visible_snapshot = visible_columns();

    let selected_count = row_order
        .iter()
        .filter(|id| selected_snapshot.contains(*id))
        .count();
    let all_selected = !rows.is_empty() && selected_count == row_order.len();

    let empty_message = empty_state.unwrap_or_else(|| "暂无数据".to_string());

    let mut selection_signal_header = selected_rows.clone();
    let row_order_for_header = row_order.clone();
    let selection_handler_header = selection_handler.clone();

    rsx! {
        div {
            class: wrapper_class,
            onclick: {
                let mut open = columns_menu_open.clone();
                move |_| {
                    if open() {
                        open.set(false);
                    }
                }
            },
            div {
                class: "ui-data-table-toolbar",
                if !toggleable_columns.is_empty() {
                    div {
                        class: "ui-data-table-columns",
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            class: "ui-data-table-columns-trigger",
                            on_click: {
                                let mut open = columns_menu_open.clone();
                                move |evt: MouseEvent| {
                                    evt.stop_propagation();
                                    let next = !open();
                                    open.set(next);
                                }
                            },
                            svg {
                                width: "16",
                                height: "16",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect { x: "3", y: "3", width: "7", height: "7" }
                                rect { x: "14", y: "3", width: "7", height: "7" }
                                rect { x: "14", y: "14", width: "7", height: "7" }
                                rect { x: "3", y: "14", width: "7", height: "7" }
                            }
                            "列控制"
                        }
                        if columns_menu_open() {
                            div {
                                class: "ui-data-table-columns-popover",
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                },
                                for column in toggleable_columns.iter() {
                                    {
                                        let column_id = column.id.clone();
                                        let column_label = column.label.clone();
                                        let mut visible_signal = visible_columns.clone();
                                        let handler = visibility_handler.clone();
                                        let column_order = column_order.clone();
                                        let min_visible = min_visible_columns;

                                        let is_visible = visible_snapshot.contains(&column_id);

                                        rsx! {
                                            label {
                                                class: "ui-data-table-columns-option",
                                                Checkbox {
                                                    checked: is_visible,
                                                    on_checked_change: move |checked| {
                                                        let mut next = visible_signal();
                                                        if checked {
                                                            if next.insert(column_id.clone()) {
                                                                visible_signal.set(next.clone());
                                                                if let Some(handler) = handler.clone() {
                                                                    let payload = column_order
                                                                        .iter()
                                                                        .filter(|id| next.contains(*id))
                                                                        .cloned()
                                                                        .collect::<Vec<_>>();
                                                                    handler.call(payload);
                                                                }
                                                            }
                                                        } else if next.len() > min_visible && next.remove(&column_id) {
                                                            visible_signal.set(next.clone());
                                                            if let Some(handler) = handler.clone() {
                                                                let payload = column_order
                                                                    .iter()
                                                                    .filter(|id| next.contains(*id))
                                                                    .cloned()
                                                                    .collect::<Vec<_>>();
                                                                handler.call(payload);
                                                            }
                                                        }
                                                    },
                                                }
                                                span { "{column_label}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if selected_count > 0 {
                    span {
                        class: "ui-data-table-selection-indicator",
                        "已选择 {selected_count} 行"
                    }
                }
            }
            div {
                class: "ui-data-table-scroll",
                Table {
                    class: Some(inner_table_class.clone()),
                    TableHeader {
                        TableRow {
                            TableHead {
                                class: Some("ui-data-table-checkbox-cell".to_string()),
                                Checkbox {
                                    checked: all_selected,
                                    disabled: rows.is_empty(),
                                    on_checked_change: move |checked| {
                                        let mut next = selection_signal_header();
                                        let handler = selection_handler_header.clone();
                                        if checked {
                                            let mut changed = false;
                                            for id in row_order_for_header.iter() {
                                                if next.insert(id.clone()) {
                                                    changed = true;
                                                }
                                            }
                                            if changed {
                                                selection_signal_header.set(next.clone());
                                                if let Some(handler) = handler.clone() {
                                                    let payload = row_order_for_header
                                                        .iter()
                                                        .filter(|id| next.contains(*id))
                                                        .cloned()
                                                        .collect::<Vec<_>>();
                                                    handler.call(payload);
                                                }
                                            }
                                        } else {
                                            let mut changed = false;
                                            for id in row_order_for_header.iter() {
                                                if next.remove(id) {
                                                    changed = true;
                                                }
                                            }
                                            if changed {
                                                selection_signal_header.set(next.clone());
                                                if let Some(handler) = handler.clone() {
                                                    let payload = row_order_for_header
                                                        .iter()
                                                        .filter(|id| next.contains(*id))
                                                        .cloned()
                                                        .collect::<Vec<_>>();
                                                    handler.call(payload);
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                            for column in columns.iter().cloned() {
                                if visible_snapshot.contains(&column.id) {
                                    TableHead { "{column.label}" }
                                }
                            }
                        }
                    }
                    TableBody {
                        for row in rows.iter().cloned() {
                            {
                                let row_id = row.id.clone();
                                let is_selected = selected_snapshot.contains(&row_id);
                                let mut selection_signal = selected_rows.clone();
                                let handler = selection_handler.clone();
                                let row_order = row_order.clone();

                                rsx! {
                                    TableRow {
                                        class: Some(if is_selected { "is-selected".to_string() } else { String::new() }),
                                        TableCell {
                                            class: Some("ui-data-table-checkbox-cell".to_string()),
                                            Checkbox {
                                                checked: is_selected,
                                                on_checked_change: move |checked| {
                                                    let mut next = selection_signal();
                                                    if checked {
                                                        if next.insert(row_id.clone()) {
                                                            selection_signal.set(next.clone());
                                                            if let Some(handler) = handler.clone() {
                                                                let payload = row_order
                                                                    .iter()
                                                                    .filter(|id| next.contains(*id))
                                                                    .cloned()
                                                                    .collect::<Vec<_>>();
                                                                handler.call(payload);
                                                            }
                                                        }
                                                    } else if next.remove(&row_id) {
                                                        selection_signal.set(next.clone());
                                                        if let Some(handler) = handler.clone() {
                                                            let payload = row_order
                                                                .iter()
                                                                .filter(|id| next.contains(*id))
                                                                .cloned()
                                                                .collect::<Vec<_>>();
                                                            handler.call(payload);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        for column in columns.iter().cloned() {
                                            if visible_snapshot.contains(&column.id) {
                                                {
                                                    let value = row
                                                        .cells
                                                        .get(&column.id)
                                                        .cloned()
                                                        .unwrap_or_default();
                                                    rsx! { TableCell { "{value}" } }
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
            if rows.is_empty() {
                div {
                    class: "ui-data-table-empty",
                    "{empty_message}"
                }
            }
        }
    }
}
